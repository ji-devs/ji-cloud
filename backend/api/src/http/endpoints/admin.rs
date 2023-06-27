use actix_files::NamedFile;
use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse,
};
use anyhow::anyhow;
use chrono::{Duration, Utc};
use futures::future::join_all;
use futures::try_join;
use ji_core::settings::RuntimeSettings;
use serde::ser::Serialize;
use serde_derive::Deserialize;
use shared::api::endpoints::admin::{
    ImportSchoolNames, InviteUsers, SearchSchoolNames, VerifySchoolName,
};
use shared::domain::admin::{
    InviteFailedReason, InviteSchoolUserFailure, InviteSchoolUsersResponse,
    SearchSchoolNamesResponse,
};
use shared::domain::billing::{SchoolId, SubscriptionTier};
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

async fn search_school_names(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Query(search): Query<<SearchSchoolNames as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<SearchSchoolNames as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Server,
> {
    let (schools, schools_count) = try_join!(
        db::account::find_school_names_with_schools(db.as_ref(), &search),
        db::account::find_school_names_with_schools_count(db.as_ref(), &search)
    )?;

    let schools = schools.into_iter().map(From::from).collect();

    Ok((
        Json(SearchSchoolNamesResponse {
            school_names: schools,
            pages: schools_count.paged(search.page_limit),
            total_schools_count: schools_count,
        }),
        http::StatusCode::OK,
    ))
}

async fn add_school_name_if_not_exists(
    pool: &PgPool,
    name: String,
) -> sqlx::Result<Option<String>> {
    if db::account::check_school_name_exists(pool, &name).await? {
        Ok(Some(name))
    } else {
        db::account::add_school_name(pool, name.into(), true).await?;
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct ImportSchoolNamesCsv {
    #[serde(rename = "Account Name")]
    school_name: String,
}

async fn import_school_names(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Json(data): Json<<ImportSchoolNames as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<ImportSchoolNames as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Server,
> {
    let names: Result<Vec<ImportSchoolNamesCsv>, _> = csv::ReaderBuilder::default()
        .has_headers(true)
        .from_reader(data.as_bytes())
        .deserialize()
        .collect();

    let mut exists = vec![];
    for name in names? {
        exists.push(add_school_name_if_not_exists(db.as_ref(), name.school_name).await?);
    }

    let exists: Vec<_> = exists.into_iter().flatten().collect();

    Ok((Json(exists), http::StatusCode::OK))
}

async fn verify_school_name(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Json(data): Json<<VerifySchoolName as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Server> {
    db::account::verify_school_name(db.as_ref(), data.school_name_id, data.verified).await?;

    Ok(HttpResponse::Ok().finish())
}

async fn invite_school_user(
    pool: &PgPool,
    school_id: &SchoolId,
    email: String,
) -> Result<Option<InviteSchoolUserFailure>, error::Server> {
    let user_id = match db::user::get_user_id_by_email(pool, &email).await? {
        Some(user_id) => {
            if let Some(account_summary) =
                db::account::get_user_account_summary(pool, &user_id).await?
            {
                match account_summary.school_id {
                    Some(_) => Err(InviteFailedReason::AssociatedWithSchool),
                    None => Err(InviteFailedReason::HasIndividualAccount),
                }
            } else {
                Ok(user_id)
            }
        }
        None => Err(InviteFailedReason::UserNotFound),
    };

    match user_id {
        Err(reason) => return Ok(Some(InviteSchoolUserFailure { email, reason })),
        Ok(user_id) => {
            let school = db::account::get_school_account_by_id(pool, &school_id)
                .await?
                .ok_or(anyhow!("School not found"))?;
            db::account::associate_user_with_account(
                pool,
                &user_id,
                &school.account_id,
                &SubscriptionTier::Pro,
                false,
                true,
            )
            .await?;
        }
    }

    Ok(None)
}

async fn invite_school_users(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Json(invite_users): Json<<InviteUsers as ApiEndpoint>::Req>,
) -> Result<(Json<<InviteUsers as ApiEndpoint>::Res>, http::StatusCode), error::Server> {
    let emails: Vec<_> = invite_users
        .data
        .lines()
        .map(|email| invite_school_user(db.as_ref(), &invite_users.school_id, email.into()))
        .collect();

    let failures: Vec<InviteSchoolUserFailure> = join_all(emails)
        .await
        .into_iter()
        .filter_map(|item| match item {
            Ok(Some(item)) => Some(Ok(item)),
            Err(error) => Some(Err(error)),
            _ => None,
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((
        Json(InviteSchoolUsersResponse { failures }),
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
        <SearchSchoolNames as ApiEndpoint>::Path::PATH,
        admin::SearchSchoolNames::METHOD
            .route()
            .to(search_school_names),
    )
    .route(
        <ImportSchoolNames as ApiEndpoint>::Path::PATH,
        ImportSchoolNames::METHOD.route().to(import_school_names),
    )
    .route(
        <VerifySchoolName as ApiEndpoint>::Path::PATH,
        VerifySchoolName::METHOD.route().to(verify_school_name),
    )
    .route(
        <InviteUsers as ApiEndpoint>::Path::PATH,
        InviteUsers::METHOD.route().to(invite_school_users),
    );
}
