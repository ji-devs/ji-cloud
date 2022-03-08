use actix_web::web::{Data, Json, ServiceConfig};
use futures::try_join;
use shared::{
    api::{endpoints::meta::Get, ApiEndpoint},
    domain::meta::MetadataResponse,
};
use sqlx::PgPool;
use tracing::instrument;

use crate::{db, error};

// TODO: Should have cache headers
/// Get a list of all available metadata of all kinds (sans categories)
#[instrument(skip(db))]
async fn get(db: Data<PgPool>) -> Result<Json<<Get as ApiEndpoint>::Res>, error::Server> {
    let (
        affiliations,
        resource_types,
        age_ranges,
        subjects,
        goals,
        image_tags,
        image_styles,
        animation_styles,
    ) = try_join!(
        db::meta::get_affiliations(&db),
        db::meta::get_additional_resources(&db),
        db::meta::get_age_ranges(&db),
        db::meta::get_subjects(&db),
        db::meta::get_goals(&db),
        db::meta::get_image_tags(&db),
        db::meta::get_image_styles(&db),
        db::meta::get_animation_styles(&db),
    )?;

    Ok(Json(MetadataResponse {
        affiliations,
        resource_types,
        age_ranges,
        subjects,
        goals,
        image_tags,
        image_styles,
        animation_styles,
    }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(Get::PATH, Get::METHOD.route().to(get));
}
