use actix_web::web::{Data, Json, Path, ServiceConfig};
use shared::{
    api::{endpoints::jig::report, ApiEndpoint},
    domain::{
        jig::{report::ReportId, JigId},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self},
    error,
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
};

/// Create a new jig report
async fn create(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    path: Path<JigId>,
    req: Json<<report::Create as ApiEndpoint>::Req>,
) -> Result<(Json<<report::Create as ApiEndpoint>::Res>, http::StatusCode), error::ReportError> {
    let jig_id = path.into_inner();

    let req = req.into_inner();

    let id = db::jig::report::create_report(&*db, jig_id, req.report_type, claims).await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Get report details for a jig
async fn get(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<(JigId, ReportId)>,
) -> Result<Json<<report::Get as ApiEndpoint>::Res>, error::ReportError> {
    let (jig_id, report_id) = path.into_inner();

    let report = db::jig::report::get_report(&db, jig_id, report_id)
        .await?
        .ok_or(error::ReportError::ResourceNotFound)?;

    Ok(Json(report))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        report::Create::PATH,
        report::Create::METHOD.route().to(create),
    )
    .route(report::Get::PATH, report::Get::METHOD.route().to(get));
}
