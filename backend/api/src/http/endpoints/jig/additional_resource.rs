use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::additional_resource, ApiEndpoint},
    domain::{
        jig::{
            additional_resource::{AdditionalResourceId, AdditionalResourceResponse},
            DraftOrLive, JigId,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Create a new additional resource.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    parent: Path<JigId>,
    req: Json<<additional_resource::Create as ApiEndpoint>::Req>,
) -> Result<
    (
        // TODO double check this
        Json<<additional_resource::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let parent_id = parent.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.into_inner();

    let id = db::jig::additional_resource::create(
        &*db,
        parent_id,
        req.resource_id,
        req.resource_content,
    )
    .await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Get an additional resource on a draft jig.
async fn get_draft(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
) -> Result<Json<<additional_resource::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let (parent_id, additional_resource_id) = path.into_inner();

    let (display_name, resource_content) = db::jig::additional_resource::get(
        &db,
        parent_id,
        DraftOrLive::Draft,
        additional_resource_id,
    )
    .await?;

    Ok(Json(AdditionalResourceResponse {
        display_name,
        resource_content,
    }))
}

/// Get an additional resource on a live jig.
async fn get_live(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
) -> Result<Json<<additional_resource::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let (parent_id, additional_resource_id) = path.into_inner();

    let (display_name, resource_content) = db::jig::additional_resource::get(
        &db,
        parent_id,
        DraftOrLive::Live,
        additional_resource_id,
    )
    .await?;

    Ok(Json(AdditionalResourceResponse {
        display_name,
        resource_content,
    }))
}

/// Delete an additional resource.
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
) -> Result<HttpResponse, error::Delete> {
    //
    let (parent_id, additional_resource_id) = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    db::jig::additional_resource::delete(&*db, parent_id, additional_resource_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        additional_resource::Create::PATH,
        additional_resource::Create::METHOD.route().to(create),
    )
    .route(
        additional_resource::GetDraft::PATH,
        additional_resource::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        additional_resource::GetLive::PATH,
        additional_resource::GetLive::METHOD.route().to(get_live),
    )
    .route(
        additional_resource::Delete::PATH,
        additional_resource::Delete::METHOD.route().to(delete),
    );
}
