use crate::db;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, ServiceConfig},
};
use shared::{
    api::{endpoints::meta::Get, ApiEndpoint},
    domain::meta::GetResponse,
    error::InternalServerError,
};
use sqlx::PgPool;

// TODO: Should have cache headers
#[api_v2_operation]
async fn get(db: Data<PgPool>) -> Result<Json<<Get as ApiEndpoint>::Res>, InternalServerError> {
    let styles = db::meta::get_style(&db).await?;
    let affiliations = db::meta::get_affiliations(&db).await?;
    let age_ranges = db::meta::get_age_ranges(&db).await?;
    let subjects = db::meta::get_subjects(&db).await?;
    let content_types = db::meta::get_content_types(&db).await?;

    Ok(Json(GetResponse {
        styles,
        affiliations,
        age_ranges,
        subjects,
        content_types,
    }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(Get::PATH, Get::METHOD.route().to(get));
}
