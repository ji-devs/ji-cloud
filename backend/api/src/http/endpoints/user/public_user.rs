use actix_web::{
    web::{Data, Json, Path, Query},
    HttpResponse,
};

use futures::try_join;
use shared::{
    api::{endpoints::user, ApiEndpoint},
    domain::{
        asset::DraftOrLive,
        course::CourseBrowseResponse,
        jig::JigBrowseResponse,
        user::{
            public_user::{
                BrowsePublicUserFollowersResponse as BrowseFollowersResponse,
                BrowsePublicUserFollowingResponse as BrowseFollowingsResponse,
                BrowsePublicUserResourcesResponse as BrowseResourcesResponse,
                BrowsePublicUserResponse, PublicUser, SearchPublicUserResponse,
            },
            UserId,
        },
    },
};

use sqlx::PgPool;

use crate::{
    db,
    error::{self, ServiceKind},
    extractor::TokenUser,
    http::endpoints::course::{DEFAULT_PAGE_LIMIT, MAX_PAGE_LIMIT},
    service::ServiceData,
};

/// Get a User
pub async fn get(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
) -> Result<Json<<user::GetPublicUser as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = path.into_inner();

    let user = db::user::public_user::get(&db, user_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(user))
}

/// Search for public user profile.
pub async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<user::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<user::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let user_id = db::user::public_user::auth_claims(&db, claims, query.user_id).await?;

    let (ids, pages, total_hits) = algolia
        .search_public_user(
            &query.q,
            query.username,
            query.name,
            user_id,
            query.language,
            query.organization,
            query.bio,
            query.persona,
            page_limit,
            query.page,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let users: Vec<_> = db::user::public_user::get_by_ids(db.as_ref(), &ids).await?;

    Ok(Json(SearchPublicUserResponse {
        users,
        pages,
        total_user_count: total_hits,
    }))
}

/// Browse Public User profiles
pub async fn browse(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    query: Option<Query<<user::BrowsePublicUser as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowsePublicUser as ApiEndpoint>::Res>, error::NotFound> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_users(
        &db,
        query.page.unwrap_or(0),
        page_limit as u64,
        query.circles.to_owned(),
    );

    let total_count_future =
        db::user::public_user::total_user_count(db.as_ref(), query.circles.to_owned());

    let (users, total_user_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_user_count / (page_limit as u64)
        + (total_user_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowsePublicUserResponse {
        users,
        pages,
        total_user_count,
    }))
}

/// Get a Public Users Jigs
pub async fn browse_user_jigs(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
    query: Option<Query<<user::BrowseUserJigs as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseUserJigs as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let privacy_level = vec![];
    let resource_types = vec![];

    let browse_future = db::jig::browse(
        &db,
        Some(user_id),
        None,
        Some(DraftOrLive::Live),
        privacy_level.to_owned(),
        Some(false),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
        None,
    );

    let total_count_future = db::jig::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        Some(false),
        Some(user_id),
        None,
        Some(DraftOrLive::Live),
        resource_types.to_owned(),
    );

    let (jigs, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(JigBrowseResponse {
        jigs,
        pages,
        total_jig_count: total_count,
    }))
}

/// Get a Public Users resources
pub async fn browse_user_resources(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
    query: Option<Query<<user::BrowseResources as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseResources as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_user_resources(
        &db,
        user_id,
        query.page.unwrap_or(0),
        page_limit as u64,
    );

    let total_count_future = db::user::public_user::total_resource_count(&db, user_id);

    let (resources, total_resource_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_resource_count / (page_limit as u64)
        + (total_resource_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowseResourcesResponse {
        resources,
        pages,
        total_resource_count,
    }))
}

/// Get a Public Users Courses
pub async fn browse_user_courses(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
    query: Option<Query<<user::BrowseCourses as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseCourses as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let privacy_level = vec![];
    let resource_types = vec![];

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::course::browse(
        &db,
        Some(user_id),
        Some(DraftOrLive::Live),
        privacy_level.to_owned(),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
    );

    let total_count_future = db::course::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        Some(user_id),
        Some(DraftOrLive::Live),
        resource_types.to_owned(),
    );

    let (courses, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(CourseBrowseResponse {
        courses,
        pages,
        total_course_count: total_count,
    }))
}

/// Follow a user
pub async fn follow(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<UserId>,
) -> Result<HttpResponse, error::NotFound> {
    let (user_id, follower_id) = (path.into_inner(), claims.user_id());

    if user_id == follower_id {
        return Err(error::NotFound::InternalServerError(anyhow::anyhow!(
            "User cannot follow self"
        )));
    }

    db::user::public_user::follow(&db, user_id, follower_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Unfollow a user
pub async fn unfollow(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<UserId>,
) -> Result<HttpResponse, error::NotFound> {
    let (user_id, follower_id) = (path.into_inner(), claims.user_id());

    db::user::public_user::unfollow(&db, user_id, follower_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a Public User's Followers
pub async fn browse_user_followers(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
    query: Option<Query<<user::BrowseFollowers as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseFollowers as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_followers(
        &db,
        user_id,
        query.page.unwrap_or(0),
        page_limit as u64,
    );

    let total_count_future = db::user::public_user::total_follower_count(db.as_ref(), user_id);

    let (followers, total_follower_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_follower_count / (page_limit as u64)
        + (total_follower_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowseFollowersResponse {
        followers,
        pages,
        total_follower_count,
    }))
}

/// Get a Public User's Followers
pub async fn browse_user_followings(
    db: Data<PgPool>,
    _auth: Option<TokenUser>,
    path: Path<UserId>,
    query: Option<Query<<user::BrowseFollowing as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseFollowing as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_following(
        &db,
        user_id,
        query.page.unwrap_or(0),
        page_limit as u64,
    );

    let total_count_future = db::user::public_user::total_following_count(db.as_ref(), user_id);

    let (followings, total_following_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_following_count / (page_limit as u64)
        + (total_following_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowseFollowingsResponse {
        followings,
        pages,
        total_following_count,
    }))
}

async fn page_limit(page_limit: Option<u32>) -> anyhow::Result<u32> {
    if let Some(limit) = page_limit {
        match limit > 0 && limit <= MAX_PAGE_LIMIT {
            true => Ok(limit),
            false => Err(anyhow::anyhow!("Page limit should be within 1-100")),
        }
    } else {
        Ok(DEFAULT_PAGE_LIMIT)
    }
}
