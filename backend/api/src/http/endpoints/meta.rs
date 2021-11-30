use actix_web::web::{Data, Json, ServiceConfig};
use shared::{
    api::{endpoints::meta::Get, ApiEndpoint},
    domain::meta::MetadataResponse,
};
use sqlx::PgPool;

use crate::{db, error};

// TODO: Should have cache headers
/// Get a list of all available metadata of all kinds (sans categories)
async fn get(db: Data<PgPool>) -> Result<Json<<Get as ApiEndpoint>::Res>, error::Server> {
    let affiliations = db::meta::get_affiliations(&db).await?;
    let resource_types = db::meta::get_additional_resources(&db).await?;
    let age_ranges = db::meta::get_age_ranges(&db).await?;
    let subjects = db::meta::get_subjects(&db).await?;
    let goals = db::meta::get_goals(&db).await?;
    let image_tags = db::meta::get_image_tags(&db).await?;
    let image_styles = db::meta::get_image_styles(&db).await?;
    let animation_styles = db::meta::get_animation_styles(&db).await?;

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
