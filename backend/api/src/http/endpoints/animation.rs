use chrono::{DateTime, Utc};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Bytes, Data, Json, Path, PayloadConfig, ServiceConfig},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints::animation, ApiEndpoint},
    domain::{
        animation::{AnimationId, AnimationResponse},
        CreateResponse,
    },
    media::{AnimationVariant, MediaLibraryKind},
};
use sqlx::{postgres::PgDatabaseError, PgPool};

use crate::{
    db, error,
    extractor::{AuthUserWithScope, ScopeManageAnimation, WrapAuthClaimsNoDb},
    s3,
};

fn check_conflict_delete(err: sqlx::Error) -> error::Delete {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            error::Delete::Conflict
        }
        _ => error::Delete::InternalServerError(err.into()),
    }
}

/// Delete an animation from the global animation library.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageAnimation>,
    req: Path<AnimationId>,
    s3: Data<s3::Client>,
) -> Result<NoContent, error::Delete> {
    let animation = req.into_inner();
    let variant = db::animation::delete(&db, animation)
        .await
        .map_err(check_conflict_delete)?;

    if let Some(variant) = variant {
        s3.delete_animation(MediaLibraryKind::Global, variant, animation)
            .await;
    }

    Ok(NoContent)
}

/// Create an animation in the global animation library.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageAnimation>,
    req: Json<<animation::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<animation::Create as ApiEndpoint>::Res>, error::CreateWithMetadata> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;
    let id = db::animation::create(
        &mut txn,
        &req.name,
        &req.description,
        req.is_premium,
        req.is_looping,
        req.publish_at.map(DateTime::<Utc>::from),
        req.variant,
    )
    .await?;

    // todo: have these exist
    // db::animation::update_metadata(
    //     &mut txn,
    //     id,
    //     nul_if_empty(&req.affiliations),
    //     nul_if_empty(&req.age_ranges),
    //     nul_if_empty(&req.styles),
    //     nul_if_empty(&req.categories),
    // )
    // .await
    // .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok(CreatedJson(CreateResponse { id }))
}

/// Upload an animation to the global animation library.
#[api_v2_operation]
async fn upload(
    db: Data<PgPool>,
    s3: Data<s3::Client>,
    _claims: AuthUserWithScope<ScopeManageAnimation>,
    Path(id): Path<AnimationId>,
    bytes: Bytes,
) -> Result<NoContent, error::Upload> {
    let variant = db::animation::get_animation_variant(db.as_ref(), id)
        .await?
        .ok_or(error::Upload::ResourceNotFound)?;

    if !matches!(variant, AnimationVariant::Gif) {
        return Err(anyhow::anyhow!("Unimplemented Animation Variant: {:?}", variant).into());
    }

    let res: Result<Bytes, error::Upload> = tokio::task::spawn_blocking(move || {
        let _original = image::load_from_memory_with_format(&bytes, image::ImageFormat::Gif)
            .map_err(|_| error::Upload::InvalidMedia)?;
        Ok(bytes)
    })
    .await?;
    let validated = res?;

    s3.upload_animation_gif(MediaLibraryKind::Global, id, validated.to_vec())
        .await?;

    Ok(NoContent)
}

/// Get an animation from the global animation library.
#[api_v2_operation]
async fn get_one(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<AnimationId>,
) -> Result<Json<<animation::Get as ApiEndpoint>::Res>, error::NotFound> {
    let metadata = db::animation::get_one(&db, req.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(AnimationResponse { metadata }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        animation::Create::PATH,
        animation::Create::METHOD.route().to(create),
    )
    .service(
        web::resource(animation::Upload::PATH)
            .app_data(PayloadConfig::default().limit(config::ANIMATION_BODY_SIZE_LIMIT))
            .route(animation::Upload::METHOD.route().to(upload)),
    )
    .route(
        animation::Get::PATH,
        animation::Get::METHOD.route().to(get_one),
    )
    // .route(
    //     animation::UpdateMetadata::PATH,
    //     animation::UpdateMetadata::METHOD.route().to(update),
    // )
    .route(
        animation::Delete::PATH,
        animation::Delete::METHOD.route().to(delete),
    );
}
