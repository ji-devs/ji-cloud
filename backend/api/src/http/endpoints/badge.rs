use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use futures::try_join;
use shared::{
    api::{endpoints::badge, ApiEndpoint},
    domain::{
        asset::UserOrMe,
        badge::{BadgeBrowseResponse, BadgeId, BadgeSearchResponse, BrowseMembersResponse},
        CreateResponse,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{self},
    error::{self, ServiceKind},
    extractor::TokenUser,
    http::endpoints::jig::page_limit,
    service::ServiceData,
};

/// Create an Badge.
async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<badge::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::CreateWithMetadata> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;

    let id = db::badge::create(
        &mut txn,
        &req.display_name,
        &req.description,
        req.thumbnail,
        claims.0.user_id,
    )
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Update a Badge
async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Option<Json<<badge::Update as ApiEndpoint>::Req>>,
    path: Path<BadgeId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();

    db::badge::authz(&*db, claims.0.user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::badge::update(
        &*db,
        id,
        req.display_name.as_deref(),
        req.description.as_deref(),
        req.thumbnail,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a Badge
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<BadgeId>,
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();

    db::badge::authz(&*db, claims.0.user_id, Some(id)).await?;

    db::badge::delete(&*db, id).await?;

    algolia.delete_badge(id).await;

    Ok(HttpResponse::NoContent().finish())
}

async fn get_one(
    db: Data<PgPool>,
    path: Path<BadgeId>,
) -> Result<Json<<badge::Get as ApiEndpoint>::Res>, error::NotFound> {
    let badge_response = db::badge::get_one(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(badge_response))
}

async fn join(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<BadgeId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    db::badge::valid_badge(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    db::badge::join_badge(&db, claims.0.user_id, id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}

async fn leave(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<BadgeId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    db::badge::valid_badge(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    db::badge::leave_badge(&db, claims.0.user_id, id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for Badges.
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<badge::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<badge::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let creator_id = auth_claims(&db, claims, query.creator_id).await?;

    let (ids, pages, total_hits) = algolia
        .search_badge(
            &query.q,
            creator_id,
            query.creator_name,
            page_limit,
            query.page,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let badges: Vec<_> = db::badge::get_by_ids(db.as_ref(), &ids).await?;

    Ok(Json(BadgeSearchResponse {
        badges,
        pages,
        total_badge_count: total_hits,
    }))
}

async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<badge::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<badge::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let creator_id = auth_claims(&db, claims, query.creator_id).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let browse_future = db::badge::browse(
        db.as_ref(),
        creator_id,
        page_limit,
        query.page.unwrap_or(0) as i32,
    );

    let total_count_future = db::badge::filtered_count(db.as_ref(), creator_id);

    let (badges, total_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_count / (page_limit as u64)
        + (total_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(BadgeBrowseResponse {
        badges,
        pages,
        total_badge_count: total_count,
    }))
}

async fn browse_members(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: Path<BadgeId>,
) -> Result<Json<<badge::BrowseMembers as ApiEndpoint>::Res>, error::NotFound> {
    let id = path.into_inner();

    db::badge::valid_badge(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    let members = db::badge::browse_badge_members(&db, id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let count = members.len() as u32;

    Ok(Json(BrowseMembersResponse { members, count }))
}

async fn auth_claims(
    db: &PgPool,
    claims: Option<TokenUser>,
    creator_id: Option<UserOrMe>,
) -> Result<Option<Uuid>, error::Auth> {
    //Check if user is logged in. If not, users cannot use UserOrMe::Me
    let id = if let Some(token) = claims {
        let id = if let Some(creator) = creator_id {
            let creator_id = match creator {
                UserOrMe::Me => Some(token.0.user_id),
                UserOrMe::User(id) => {
                    if !sqlx::query!(
                        //language=SQL
                        r#"
            select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
                "#,
                        id
                    )
                    .fetch_one(db)
                    .await?
                    .exists
                    {
                        return Err(error::Auth::ResourceNotFound(
                            "Creator Id does not exist".to_string(),
                        ));
                    }

                    Some(id)
                }
            };
            creator_id
        } else {
            None
        };
        id
    } else {
        let id = if let Some(creator) = creator_id {
            let creator = match creator {
                UserOrMe::Me => return Err(error::Auth::Forbidden),
                UserOrMe::User(id) => {
                    if !sqlx::query!(
                        //language=SQL
                        r#"
                select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
                    "#,
                        id
                    )
                    .fetch_one(db)
                    .await?
                    .exists
                    {
                        return Err(error::Auth::ResourceNotFound(
                            "Creator Id does not exist".to_string(),
                        ));
                    }

                    Some(id)
                }
            };
            creator
        } else {
            None
        };
        id
    };

    Ok(id)
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        badge::Create::PATH,
        badge::Create::METHOD.route().to(create),
    )
    .route(
        badge::Browse::PATH,
        badge::Browse::METHOD.route().to(browse),
    )
    .route(
        badge::Search::PATH,
        badge::Search::METHOD.route().to(search),
    )
    .route(
        badge::BrowseMembers::PATH,
        badge::BrowseMembers::METHOD.route().to(browse_members),
    )
    .route(
        badge::Update::PATH,
        badge::Update::METHOD.route().to(update),
    )
    .route(
        badge::Delete::PATH,
        badge::Delete::METHOD.route().to(delete),
    )
    .route(badge::Get::PATH, badge::Get::METHOD.route().to(get_one))
    .route(
        badge::JoinBadge::PATH,
        badge::JoinBadge::METHOD.route().to(join),
    )
    .route(
        badge::LeaveBadge::PATH,
        badge::LeaveBadge::METHOD.route().to(leave),
    );
}
