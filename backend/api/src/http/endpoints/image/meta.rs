use crate::db;
use actix_web::web::{Data, Json, ServiceConfig};
use shared::{
    api::{
        endpoints::image::meta::{GetAffiliations, GetAgeRange, GetStyle},
        ApiEndpoint,
    },
    domain::image::meta::{AffiliationResponse, AgeRangeResponse, StyleResponse},
    error::InternalServerError,
};
use sqlx::PgPool;

// TODO: All of these should have Cache headers
async fn get_style(
    db: Data<PgPool>,
) -> Result<Json<<GetStyle as ApiEndpoint>::Res>, InternalServerError> {
    let styles = db::image::meta::get_style(&db).await?;
    Ok(Json(StyleResponse { styles }))
}

async fn get_affiliations(
    db: Data<PgPool>,
) -> Result<Json<<GetAffiliations as ApiEndpoint>::Res>, InternalServerError> {
    let affiliations = db::image::meta::get_affiliations(&db).await?;
    Ok(Json(AffiliationResponse { affiliations }))
}

async fn get_age_range(
    db: Data<PgPool>,
) -> Result<Json<<GetAgeRange as ApiEndpoint>::Res>, InternalServerError> {
    let age_ranges = db::image::meta::get_age_ranges(&db).await?;
    Ok(Json(AgeRangeResponse { age_ranges }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(GetStyle::PATH, GetStyle::METHOD.route().to(get_style))
        .route(
            GetAffiliations::PATH,
            GetAffiliations::METHOD.route().to(get_affiliations),
        )
        .route(
            GetAgeRange::PATH,
            GetAgeRange::METHOD.route().to(get_age_range),
        );
}
