use actix_web::{
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use futures::try_join;
use shared::{
    api::{endpoints::user, ApiEndpoint},
    domain::user::public_user::{
        BrowsePublicUserCoursesResponse as BrowseCoursesResponse,
        BrowsePublicUserFollowersResponse as BrowseFollowersResponse,
        BrowsePublicUserFollowingResponse as BrowseFollowingsResponse,
        BrowsePublicUserJigsResponse as BrowseJigsResponse,
        BrowsePublicUserResourcesResponse as BrowseResourcesResponse, BrowsePublicUserResponse,
        PublicUser,
    },
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db, error,
    extractor::TokenUser,
    http::endpoints::course::{DEFAULT_PAGE_LIMIT, MAX_PAGE_LIMIT},
};

/// Get a User
pub async fn get(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<Uuid>,
) -> Result<Json<<user::GetPublicUser as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = path.into_inner();

    let user: PublicUser = db::user::public_user::get(&db, user_id).await?;

    Ok(Json(user))
}

/// Get a User
pub async fn browse(
    db: Data<PgPool>,
    _auth: TokenUser,
    query: Option<Query<<user::BrowsePublicUser as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowsePublicUser as ApiEndpoint>::Res>, error::NotFound> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future =
        db::user::public_user::browse_users(&db, query.page.unwrap_or(0), page_limit as u64);

    let total_count_future = db::user::public_user::total_user_count(db.as_ref());

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
    _auth: TokenUser,
    path: Path<Uuid>,
    query: Option<Query<<user::BrowseUserJigs as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseUserJigs as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_user_jigs(
        &db,
        user_id,
        query.page.unwrap_or(0),
        page_limit as u64,
    );

    let total_count_future = db::user::public_user::total_jig_count(db.as_ref(), user_id);

    let (jigs, total_jig_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_jig_count / (page_limit as u64)
        + (total_jig_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowseJigsResponse {
        jigs,
        pages,
        total_jig_count,
    }))
}

/// Get a Public Users resources
pub async fn browse_user_resources(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<Uuid>,
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

    let total_count_future = db::user::public_user::total_resource_count(db.as_ref(), user_id);

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
    _auth: TokenUser,
    path: Path<Uuid>,
    query: Option<Query<<user::BrowseCourses as ApiEndpoint>::Req>>,
) -> Result<Json<<user::BrowseCourses as ApiEndpoint>::Res>, error::NotFound> {
    let (query, user_id) = (
        query.map_or_else(Default::default, Query::into_inner),
        path.into_inner(),
    );

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let browse_future = db::user::public_user::browse_user_courses(
        &db,
        user_id,
        query.page.unwrap_or(0),
        page_limit as u64,
    );

    let total_count_future = db::user::public_user::total_course_count(db.as_ref(), user_id);

    let (courses, total_course_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_course_count / (page_limit as u64)
        + (total_course_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BrowseCoursesResponse {
        courses,
        pages,
        total_course_count,
    }))
}

/// Follow a user
pub async fn follow(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<Uuid>,
) -> Result<HttpResponse, error::NotFound> {
    let (user_id, follower_id) = (path.into_inner(), claims.0.user_id);

    db::user::public_user::follow(&db, user_id, follower_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Unfollow a user
pub async fn unfollow(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<Uuid>,
) -> Result<HttpResponse, error::NotFound> {
    let (user_id, follower_id) = (path.into_inner(), claims.0.user_id);

    db::user::public_user::unfollow(&db, user_id, follower_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a Public User's Followers
pub async fn browse_user_followers(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<Uuid>,
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
    _auth: TokenUser,
    path: Path<Uuid>,
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
