use paperclip::actix::{
    api_v2_operation,
    web::{Data, Path, ServiceConfig},
    NoContent,
};
use shared::{
    api::{endpoints::animation, ApiEndpoint},
    domain::animation::AnimationId,
    media::MediaLibraryKind,
};
use sqlx::{postgres::PgDatabaseError, PgPool};

use crate::{
    db, error,
    extractor::{AuthUserWithScope, ScopeManageAnimation},
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

/// Delete an image from the global image library.
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

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg
        // .route(
        //     animation::Create::PATH,
        //     animation::Create::METHOD.route().to(create),
        // )
        // .service(
        //     web::resource(animation::Upload::PATH)
        //         .app_data(PayloadConfig::default().limit(config::animation_BODY_SIZE_LIMIT))
        //         .route(animation::Upload::METHOD.route().to(upload)),
        // )
        // .route(animation::Get::PATH, animation::Get::METHOD.route().to(get_one))
        // .route(
        //     animation::UpdateMetadata::PATH,
        //     animation::UpdateMetadata::METHOD.route().to(update),
        // )
        .route(
            animation::Delete::PATH,
            animation::Delete::METHOD.route().to(delete),
        );
}
