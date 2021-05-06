use actix_web::web::Query;
use chrono::{DateTime, Utc};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json, ServiceConfig},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{
            Jig, JigBrowseResponse, JigCreateRequest, JigId, JigResponse, JigSearchResponse,
            UserOrMe,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db,
    error::{self, ServiceKind},
    extractor::TokenUser,
    service::ServiceData,
};

/// Create a jig.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<jig::Create as ApiEndpoint>::Req>>,
) -> Result<CreatedJson<<jig::Create as ApiEndpoint>::Res>, error::CreateWithMetadata> {
    let db = db.as_ref();
    db::jig::authz(db, auth.0.user_id, None).await?;

    let req = req.map_or_else(JigCreateRequest::default, Json::into_inner);
    let creator_id = auth.0.user_id;

    let language = match req.language {
        Some(lang) => lang,
        None => {
            sqlx::query!(
                "select language from user_profile where user_id = $1",
                auth.0.user_id
            )
            .fetch_one(db)
            .await?
            .language
        }
    };

    let id = db::jig::create(
        &*db,
        req.display_name.as_deref(),
        &req.goals,
        &req.categories,
        &req.age_ranges,
        &req.affiliations,
        creator_id,
        req.publish_at.map(DateTime::<Utc>::from),
        &language,
    )
    .await
    .map_err(db::meta::handle_metadata_err)?;

    Ok(CreatedJson(CreateResponse { id }))
}

/// Clone a jig
#[api_v2_operation]
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<JigId>,
) -> Result<CreatedJson<<jig::Create as ApiEndpoint>::Res>, error::NotFound> {
    db::jig::authz(&*db, claims.0.user_id, None).await?;

    match db::jig::clone(&*db, parent.into_inner(), claims.0.user_id).await? {
        Some(id) => Ok(CreatedJson(CreateResponse { id })),
        None => Err(error::NotFound::ResourceNotFound),
    }
}

/// Delete a jig.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<NoContent, error::Delete> {
    let id = path.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    db::jig::delete(&*db, id).await?;

    algolia.delete_jig(id).await;

    Ok(NoContent)
}

/// Update a jig.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Option<Json<<jig::Update as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<NoContent, error::UpdateWithMetadata> {
    let id = path.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::jig::update(
        &*db,
        id,
        req.display_name.as_deref(),
        req.author_id,
        req.goals.as_deref(),
        req.categories.as_deref(),
        req.age_ranges.as_deref(),
        req.affiliations.as_deref(),
        req.publish_at.map(|it| it.map(DateTime::<Utc>::from)),
        req.language.as_deref(),
    )
    .await?;

    Ok(NoContent)
}

/// Get a jig.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Get as ApiEndpoint>::Res>, error::NotFound> {
    let jig = db::jig::get(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(JigResponse { jig }))
}

#[api_v2_operation]
async fn browse(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Option<Query<<jig::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let author_id = query.author_id.map(|it| match it {
        UserOrMe::Me => claims.0.user_id,
        UserOrMe::User(id) => id,
    });

    db::jig::authz_list(&*db, claims.0.user_id, author_id).await?;

    let jigs = db::jig::list(
        db.as_ref(),
        query.is_published,
        author_id,
        query.page.unwrap_or(0) as i32,
    )
    .await?;

    let total_count = db::jig::filtered_count(db.as_ref(), query.is_published, author_id).await?;

    let pages = (total_count / 20 + (total_count % 20 != 0) as u64) as u32;

    Ok(Json(JigBrowseResponse {
        jigs,
        pages,
        total_jig_count: total_count,
    }))
}

/// Search for jigs.
#[api_v2_operation]
async fn search(
    db: Data<PgPool>,
    algolia: ServiceData<crate::algolia::Client>,
    _claims: TokenUser,
    query: Option<Query<<jig::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (ids, pages, total_hits) = algolia
        .search_jig(
            &query.q,
            query.page,
            query.is_published,
            &query.age_ranges,
            &query.affiliations,
            &query.categories,
            &query.goals,
            query.author,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let jigs: Vec<_> = db::jig::get_by_ids(db.as_ref(), &ids)
        .await?
        .into_iter()
        .map(|jig: Jig| JigResponse { jig })
        .collect();

    Ok(Json(JigSearchResponse {
        jigs,
        pages,
        total_image_count: total_hits,
    }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(jig::Browse::PATH, jig::Browse::METHOD.route().to(browse))
        .route(jig::Get::PATH, jig::Get::METHOD.route().to(get))
        .route(jig::Clone::PATH, jig::Clone::METHOD.route().to(clone))
        .route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(jig::Search::PATH, jig::Search::METHOD.route().to(search))
        .route(jig::Update::PATH, jig::Update::METHOD.route().to(update))
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete));
}
