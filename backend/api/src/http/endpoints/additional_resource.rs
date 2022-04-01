use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::additional_resource, ApiEndpoint},
    domain::{
        additional_resource::{AdditionalResource, AdditionalResourceId, JigOrPath, JigOrPathId},
        jig::{DraftOrLive, JigId},
        learning_path::LearningPathId,
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Create a new additional resource.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Json<<additional_resource::Create as ApiEndpoint>::Req>,
) -> Result<
    (
        // TODO double check this
        Json<<additional_resource::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let req = req.into_inner();
    let id = match req.jig_or_path_id {
        JigOrPathId::JigId(id) => {
            db::jig::authz(&*db, auth.0.user_id, Some(id)).await?;

            db::jig::additional_resource::create(
                &*db,
                id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?
        }
        JigOrPathId::LearningPathId(id) => {
            db::learning_path::authz(&*db, auth.0.user_id, Some(id)).await?;

            db::learning_path::additional_resource::create(
                &*db,
                id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?
        }
    };

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Update an additional resource.
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<AdditionalResourceId>,
    req: Json<<additional_resource::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let additional_resource_id = path.into_inner();
    let req = req.into_inner();

    match req.jig_or_path_id {
        Some(JigOrPathId::JigId(jig_id)) => {
            db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

            db::jig::additional_resource::update(
                &*db,
                jig_id,
                DraftOrLive::Draft,
                additional_resource_id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?;
        }
        Some(JigOrPathId::LearningPathId(learning_path_id)) => {
            db::learning_path::authz(&*db, auth.0.user_id, Some(learning_path_id)).await?;

            db::learning_path::additional_resource::update(
                &*db,
                learning_path_id,
                DraftOrLive::Draft,
                additional_resource_id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?;
        }
        None => {
            return Err(error::Auth::InternalServerError(anyhow::anyhow!(
                "must specify a JIG or Learning Path Id"
            )))
        }
    };

    Ok(HttpResponse::NoContent().finish())
}

/// Get an additional resource on a draft JIG or Learning Plan.
async fn get_draft(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<AdditionalResourceId>,
    query: Query<<additional_resource::GetDraft as ApiEndpoint>::Req>,
) -> Result<Json<<additional_resource::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let additional_resource_id = path.into_inner();

    let query = query.into_inner();

    let (display_name, resource_type_id, resource_content) = match query.jig_or_path {
        JigOrPath::Jig => {
            db::jig::additional_resource::get(
                &db,
                JigId(query.id),
                DraftOrLive::Draft,
                additional_resource_id,
            )
            .await?
        }
        JigOrPath::LearningPath => {
            db::learning_path::additional_resource::get(
                &db,
                LearningPathId(query.id),
                DraftOrLive::Draft,
                additional_resource_id,
            )
            .await?
        }
    };

    Ok(Json(AdditionalResource {
        id: additional_resource_id,
        display_name,
        resource_type_id,
        resource_content,
    }))
}

/// Get an additional resource on a live JIG or Learning Path.
async fn get_live(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<AdditionalResourceId>,
    query: Query<<additional_resource::GetLive as ApiEndpoint>::Req>,
) -> Result<Json<<additional_resource::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let additional_resource_id = path.into_inner();

    let query = query.into_inner();

    let (display_name, resource_type_id, resource_content) = match query.jig_or_path {
        JigOrPath::Jig => {
            db::jig::additional_resource::get(
                &db,
                JigId(query.id),
                DraftOrLive::Live,
                additional_resource_id,
            )
            .await?
        }
        JigOrPath::LearningPath => {
            db::learning_path::additional_resource::get(
                &db,
                LearningPathId(query.id),
                DraftOrLive::Live,
                additional_resource_id,
            )
            .await?
        }
    };

    Ok(Json(AdditionalResource {
        id: additional_resource_id,
        display_name,
        resource_type_id,
        resource_content,
    }))
}

/// Delete an additional resource.
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<AdditionalResourceId>,
    req: Json<<additional_resource::Delete as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Delete> {
    let additional_resource_id = path.into_inner();
    let req = req.into_inner();

    match req.jig_or_path {
        JigOrPath::Jig => {
            let jig_id = JigId(req.id);
            db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

            db::jig::additional_resource::delete(&*db, jig_id, additional_resource_id).await?;
        }
        JigOrPath::LearningPath => {
            let learning_path_id = LearningPathId(req.id);

            db::learning_path::authz(&*db, auth.0.user_id, Some(learning_path_id)).await?;

            db::learning_path::additional_resource::delete(
                &*db,
                learning_path_id,
                additional_resource_id,
            )
            .await?;
        }
    };

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        additional_resource::Create::PATH,
        additional_resource::Create::METHOD.route().to(create),
    )
    .route(
        additional_resource::Update::PATH,
        additional_resource::Update::METHOD.route().to(update),
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
