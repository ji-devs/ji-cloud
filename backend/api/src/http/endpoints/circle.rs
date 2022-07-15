use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use futures::try_join;
use shared::{
    api::{endpoints::circle, ApiEndpoint},
    domain::{
        asset::UserOrMe,
        circle::{BrowseMembersResponse, CircleBrowseResponse, CircleId, CircleSearchResponse},
        user::UserId,
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self},
    error::{self, ServiceKind},
    extractor::TokenUser,
    http::endpoints::jig::page_limit,
    service::ServiceData,
};

/// Create an Circle.
async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<circle::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::CreateWithMetadata> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;

    let id = db::circle::create(
        &mut txn,
        &req.display_name,
        &req.description,
        req.image,
        UserId(claims.0.user_id),
    )
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Update a Circle
async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Option<Json<<circle::Update as ApiEndpoint>::Req>>,
    path: Path<CircleId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();

    db::circle::authz(&*db, UserId(claims.0.user_id), Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::circle::update(
        &*db,
        id,
        req.display_name.as_deref(),
        req.description.as_deref(),
        req.image,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a Circle
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<CircleId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();

    db::circle::authz(&*db, UserId(claims.0.user_id), Some(id)).await?;

    db::circle::delete(&*db, id).await?;

    algolia.delete_circle(id).await;

    Ok(HttpResponse::NoContent().finish())
}

async fn get_one(
    db: Data<PgPool>,
    path: Path<CircleId>,
) -> Result<Json<<circle::Get as ApiEndpoint>::Res>, error::NotFound> {
    let circle_response = db::circle::get_one(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(circle_response))
}

async fn join(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<CircleId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    db::circle::valid_circle(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    db::circle::join_circle(&db, UserId(claims.0.user_id), id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}

async fn leave(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<CircleId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    db::circle::valid_circle(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    db::circle::leave_circle(&db, UserId(claims.0.user_id), id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for Circles.
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<circle::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<circle::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let creator_id = auth_claims(&db, claims, query.creator_id).await?;

    let (ids, pages, total_hits) = algolia
        .search_circle(
            &query.q,
            creator_id,
            query.creator_name,
            page_limit,
            query.page,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let circles: Vec<_> = db::circle::get_by_ids(db.as_ref(), &ids).await?;

    Ok(Json(CircleSearchResponse {
        circles,
        pages,
        total_circle_count: total_hits,
    }))
}

async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<circle::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<circle::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let creator_id = auth_claims(&db, claims, query.creator_id).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let browse_future = db::circle::browse(
        db.as_ref(),
        creator_id,
        page_limit,
        query.page.unwrap_or(0) as i32,
    );

    let total_count_future = db::circle::filtered_count(db.as_ref(), creator_id);

    let (circles, total_count) = try_join!(browse_future, total_count_future,)?;

    let pages = (total_count / (page_limit as u64)
        + (total_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(CircleBrowseResponse {
        circles,
        pages,
        total_circle_count: total_count,
    }))
}

async fn browse_members(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: Path<CircleId>,
) -> Result<Json<<circle::BrowseMembers as ApiEndpoint>::Res>, error::NotFound> {
    let id = path.into_inner();

    db::circle::valid_circle(&db, id)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    let members = db::circle::browse_circle_members(&db, id)
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    let count = members.len() as u32;

    Ok(Json(BrowseMembersResponse { members, count }))
}

async fn auth_claims(
    db: &PgPool,
    claims: Option<TokenUser>,
    creator_id: Option<UserOrMe>,
) -> Result<Option<UserId>, error::Auth> {
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
        id.map(|x| UserId(x))
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
        id.map(|x| UserId(x))
    };

    Ok(id)
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        circle::Create::PATH,
        circle::Create::METHOD.route().to(create),
    )
    .route(
        circle::Browse::PATH,
        circle::Browse::METHOD.route().to(browse),
    )
    .route(
        circle::Search::PATH,
        circle::Search::METHOD.route().to(search),
    )
    .route(
        circle::BrowseMembers::PATH,
        circle::BrowseMembers::METHOD.route().to(browse_members),
    )
    .route(
        circle::Update::PATH,
        circle::Update::METHOD.route().to(update),
    )
    .route(
        circle::Delete::PATH,
        circle::Delete::METHOD.route().to(delete),
    )
    .route(circle::Get::PATH, circle::Get::METHOD.route().to(get_one))
    .route(
        circle::JoinCircle::PATH,
        circle::JoinCircle::METHOD.route().to(join),
    )
    .route(
        circle::LeaveCircle::PATH,
        circle::LeaveCircle::METHOD.route().to(leave),
    );
}
