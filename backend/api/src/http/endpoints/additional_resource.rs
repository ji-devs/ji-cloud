use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::additional_resource, ApiEndpoint, PathParts},
    domain::{
        additional_resource::{AdditionalResource, AdditionalResourceId},
        asset::{AssetId, DraftOrLive},
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
        Json<<additional_resource::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let req = req.into_inner();
    let user_id = auth.user_id();

    let id: AdditionalResourceId = match req.asset_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, user_id, Some(jig_id)).await?;

            db::jig::additional_resource::create(
                &*db,
                jig_id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, user_id, Some(course_id)).await?;

            db::course::additional_resource::create(
                &*db,
                course_id,
                req.display_name,
                req.resource_type_id,
                req.resource_content,
            )
            .await?
        }
        AssetId::ResourceId(resource_id) => {
            db::resource::authz(&*db, user_id, Some(resource_id)).await?;

            db::resource::additional_resource::create(
                &*db,
                resource_id,
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
    let user_id = auth.user_id();

    if let Some(asset_id) = req.asset_id {
        match asset_id {
            AssetId::JigId(jig_id) => {
                db::jig::authz(&*db, user_id, Some(jig_id)).await?;

                db::jig::additional_resource::update(
                    &*db,
                    jig_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                    req.display_name,
                    req.resource_type_id,
                    req.resource_content,
                )
                .await?
            }
            AssetId::CourseId(course_id) => {
                db::course::authz(&*db, user_id, Some(course_id)).await?;

                db::course::additional_resource::update(
                    &*db,
                    course_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                    req.display_name,
                    req.resource_type_id,
                    req.resource_content,
                )
                .await?;
            }
            AssetId::ResourceId(resource_id) => {
                db::resource::authz(&*db, user_id, Some(resource_id)).await?;

                db::resource::additional_resource::update(
                    &*db,
                    resource_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                    req.display_name,
                    req.resource_type_id,
                    req.resource_content,
                )
                .await?;
            }
        }
    } else {
        return Err(error::Auth::InternalServerError(anyhow::anyhow!(
            "Must use existing asset"
        )));
    };

    Ok(HttpResponse::NoContent().finish())
}

/// Get an additional resource on a draft JIG or Course.
async fn get_draft(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<AdditionalResourceId>,
    query: Option<Query<<additional_resource::GetDraft as ApiEndpoint>::Req>>,
) -> Result<Json<<additional_resource::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let additional_resource_id = path.into_inner();

    let asset = query.map_or_else(Default::default, Query::into_inner);

    let (display_name, resource_type_id, resource_content) = if let Some(id) = asset.asset_id {
        match id {
            AssetId::JigId(jig_id) => {
                db::jig::additional_resource::get(
                    &db,
                    jig_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                )
                .await?
            }
            AssetId::CourseId(course_id) => {
                db::course::additional_resource::get(
                    &db,
                    course_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                )
                .await?
            }
            AssetId::ResourceId(resource_id) => {
                db::resource::additional_resource::get(
                    &db,
                    resource_id,
                    DraftOrLive::Draft,
                    additional_resource_id,
                )
                .await?
            }
        }
    } else {
        return Err(error::NotFound::InternalServerError(anyhow::anyhow!(
            "Must use existing asset"
        )));
    };

    Ok(Json(AdditionalResource {
        id: additional_resource_id,
        display_name,
        resource_type_id,
        resource_content,
    }))
}

/// Get an additional resource on a live JIG or Course.
async fn get_live(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<AdditionalResourceId>,
    query: Option<Query<<additional_resource::GetLive as ApiEndpoint>::Req>>,
) -> Result<Json<<additional_resource::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let additional_resource_id = path.into_inner();

    let asset = query.map_or_else(Default::default, Query::into_inner);

    let (display_name, resource_type_id, resource_content) = if let Some(id) = asset.asset_id {
        match id {
            AssetId::JigId(jig_id) => {
                db::jig::additional_resource::get(
                    &db,
                    jig_id,
                    DraftOrLive::Live,
                    additional_resource_id,
                )
                .await?
            }
            AssetId::CourseId(course_id) => {
                db::course::additional_resource::get(
                    &db,
                    course_id,
                    DraftOrLive::Live,
                    additional_resource_id,
                )
                .await?
            }
            AssetId::ResourceId(resource_id) => {
                db::resource::additional_resource::get(
                    &db,
                    resource_id,
                    DraftOrLive::Live,
                    additional_resource_id,
                )
                .await?
            }
        }
    } else {
        return Err(error::NotFound::InternalServerError(anyhow::anyhow!(
            "Must use existing asset"
        )));
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
    let req = req.into_inner();
    let additional_resource_id = path.into_inner();
    let user_id = auth.user_id();

    if let Some(id) = req.asset_id {
        match id {
            AssetId::JigId(jig_id) => {
                db::jig::authz(&*db, user_id, Some(jig_id)).await?;

                db::jig::additional_resource::delete(&*db, jig_id, additional_resource_id).await?;
            }
            AssetId::CourseId(course_id) => {
                db::course::authz(&*db, user_id, Some(course_id)).await?;

                db::course::additional_resource::delete(&*db, course_id, additional_resource_id)
                    .await?;
            }
            AssetId::ResourceId(resource_id) => {
                db::resource::authz(&*db, user_id, Some(resource_id)).await?;

                db::resource::additional_resource::delete(
                    &*db,
                    resource_id,
                    additional_resource_id,
                )
                .await?;
            }
        }
    } else {
        return Err(error::Delete::InternalServerError(anyhow::anyhow!(
            "Must use existing asset"
        )));
    };

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <additional_resource::Create as ApiEndpoint>::Path::PATH,
        additional_resource::Create::METHOD.route().to(create),
    )
    .route(
        <additional_resource::Update as ApiEndpoint>::Path::PATH,
        additional_resource::Update::METHOD.route().to(update),
    )
    .route(
        <additional_resource::GetDraft as ApiEndpoint>::Path::PATH,
        additional_resource::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <additional_resource::GetLive as ApiEndpoint>::Path::PATH,
        additional_resource::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <additional_resource::Delete as ApiEndpoint>::Path::PATH,
        additional_resource::Delete::METHOD.route().to(delete),
    );
}
