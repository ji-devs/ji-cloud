use crate::db;
use actix_web::web::{Data, Json, ServiceConfig};
use shared::{
    api::{endpoints::image::meta::Get, ApiEndpoint},
    domain::image::meta::GetResponse,
    error::InternalServerError,
};
use sqlx::PgPool;

// TODO: Should have cache headers
async fn get(db: Data<PgPool>) -> Result<Json<<Get as ApiEndpoint>::Res>, InternalServerError> {
    let styles = db::image::meta::get_style(&db).await?;
    let affiliations = db::image::meta::get_affiliations(&db).await?;
    let age_ranges = db::image::meta::get_age_ranges(&db).await?;

    Ok(Json(GetResponse {
        styles,
        affiliations,
        age_ranges,
    }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(Get::PATH, Get::METHOD.route().to(get));
}
