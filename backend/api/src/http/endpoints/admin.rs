use actix_files::NamedFile;
use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse,
};
use chrono::{Duration, Utc};
use ji_core::settings::RuntimeSettings;
use serde::ser::Serialize;
use shared::api::endpoints::admin::ListSchoolNames;
use shared::domain::admin::ListSchoolNamesResponse;
use shared::{
    api::{
        endpoints::admin::{self, CreateUpdateSubscriptionPlan},
        ApiEndpoint, PathParts,
    },
    domain::{
        admin::{ExportDataRequest, ExportType},
        session::NewSessionResponse,
        user::UserId,
    },
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserNoCsrfWithScope, TokenUserWithScope},
    token::{create_auth_token, SessionMask},
};

/// Impersonate another user
async fn impersonate(
    auth: TokenUserWithScope<ScopeAdmin>,
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: Path<UserId>,
) -> actix_web::Result<HttpResponse, error::UserNotFound> {
    let user_id = user.into_inner();

    let exists = db::user::exists(&db, user_id).await?;

    if !exists {
        return Err(error::UserNotFound::UserNotFound);
    }

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    let session = db::session::create(
        &mut *db.acquire().await?,
        user_id,
        Some(&(Utc::now() + login_ttl)),
        SessionMask::GENERAL_API,
        Some(auth.claims.user_id),
    )
    .await?;

    let (csrf, cookie) = create_auth_token(
        &settings.token_secret,
        settings.is_local(),
        login_ttl,
        &session,
    )?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(NewSessionResponse { csrf }))
}

async fn export_data_by_type(
    db: &PgPool,
    query: ExportDataRequest,
) -> Result<Vec<impl Serialize>, error::Server> {
    let data = match query.export_type {
        ExportType::Profiles => {
            db::user::user_profiles_by_date_range(
                &db,
                query.date_filter_type,
                query.from_date,
                query.to_date,
            )
            .await?
        }
    };

    Ok(data)
}

async fn export_data(
    _auth: TokenUserNoCsrfWithScope<ScopeAdmin>,
    req: HttpRequest,
    db: Data<PgPool>,
    query: Query<<admin::ExportData as ApiEndpoint>::Req>,
) -> actix_web::Result<HttpResponse, error::Server> {
    let filename = {
        let mut file_parts = vec!["jigzi".to_string()];
        file_parts.push(format!("{}_export", query.export_type).to_lowercase());

        if let Some(date) = query.from_date {
            file_parts.push(format!("{}", date.format("%Y-%m-%d")));
        }

        if let Some(date) = query.to_date {
            file_parts.push(format!("{}", date.format("%Y-%m-%d")));
        }

        let mut filename = std::env::temp_dir();
        filename.push(format!("{}.csv", file_parts.join("_")));
        filename
    };

    let data = export_data_by_type(&db, query.into_inner()).await?;

    let file = std::fs::File::create(&filename)?;

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(file);

    for profile in data.iter() {
        writer.serialize(&profile)?;
    }

    writer.flush()?;

    let file = NamedFile::from_file(std::fs::File::open(&filename)?, &filename)?;

    std::fs::remove_file(&filename)?;

    Ok(file.into_response(&req))
}

async fn create_or_update_subscription_plan(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    req: Json<<CreateUpdateSubscriptionPlan as ApiEndpoint>::Req>,
) -> actix_web::Result<HttpResponse, error::Server> {
    db::billing::upsert_subscription_plan(&db, req.into_inner()).await?;

    Ok(HttpResponse::Created().finish())
}

async fn list_school_names(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    req: Json<<ListSchoolNames as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<ListSchoolNames as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Server,
> {
    let schools = db::account::get_school_names_with_schools(db.as_ref(), req.0.verified).await?;

    Ok((
        Json(ListSchoolNamesResponse { schools }),
        http::StatusCode::OK,
    ))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <admin::Impersonate as ApiEndpoint>::Path::PATH,
        admin::Impersonate::METHOD.route().to(impersonate),
    )
    .route(
        <admin::ExportData as ApiEndpoint>::Path::PATH,
        admin::ExportData::METHOD.route().to(export_data),
    )
    .route(
        <CreateUpdateSubscriptionPlan as ApiEndpoint>::Path::PATH,
        admin::CreateUpdateSubscriptionPlan::METHOD
            .route()
            .to(create_or_update_subscription_plan),
    )
    .route(
        <ListSchoolNames as ApiEndpoint>::Path::PATH,
        admin::ListSchoolNames::METHOD.route().to(list_school_names),
    );
}
