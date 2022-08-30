use actix_web::web::{Data, Json, Path, ServiceConfig};
use core::settings::RuntimeSettings;
use sendgrid::v3::Email;
use shared::{
    api::{endpoints::resource::report, ApiEndpoint, PathParts},
    domain::{
        resource::{report::ReportId, report::ResourceReportEmail, ResourceId},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self},
    error,
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
    service::{mail, ServiceData},
};
use uuid::Uuid;

/// Create a new resource report and send the report email to admin
async fn create(
    config: Data<RuntimeSettings>,
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    mail: ServiceData<mail::Client>,
    path: Path<ResourceId>,
    req: Json<<report::Create as ApiEndpoint>::Req>,
) -> Result<(Json<<report::Create as ApiEndpoint>::Res>, http::StatusCode), error::ReportError> {
    let resource_id = path.into_inner();
    let req = req.into_inner();

    let user_id: Option<Uuid> = if let Some(user) = claims {
        Some(user.0.user_id)
    } else {
        None
    };

    let id =
        db::resource::report::create_report(&*db, resource_id, req.report_type, user_id).await?;

    let mut txn = db.begin().await?;

    let report_info = db::resource::report::get_report_email(&mut txn, resource_id, id)
        .await?
        .ok_or(error::ReportError::ResourceNotFound)?;

    send_report(
        &mail,
        resource_id,
        report_info,
        &config.remote_target().jigzi_info_email(),
        &config.remote_target().pages_url(),
    )
    .await?;

    txn.commit().await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Get report details for a resource
async fn get(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<(ResourceId, ReportId)>,
) -> Result<Json<<report::Get as ApiEndpoint>::Res>, error::ReportError> {
    let (resource_id, report_id) = path.into_inner();

    let report = db::resource::report::get_report(&db, resource_id, report_id)
        .await?
        .ok_or(error::ReportError::ResourceNotFound)?;

    Ok(Json(report))
}

async fn send_report(
    mail: &mail::Client,
    resource_id: ResourceId,
    report: ResourceReportEmail,
    email_address: &str,
    pages_url: &str,
) -> Result<(), error::ReportError> {
    let email_link = format!("{}/resource/play/{}", pages_url, resource_id.0);

    mail.send_resource_report_email(Email::new(email_address), report, email_link)
        .await?;

    Ok(())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <report::Create as ApiEndpoint>::Path::PATH,
        report::Create::METHOD.route().to(create),
    )
    .route(
        <report::Get as ApiEndpoint>::Path::PATH,
        report::Get::METHOD.route().to(get),
    );
}
