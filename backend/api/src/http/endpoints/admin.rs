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
    CreateSchoolName, DeleteUserAccount, GetSchoolNames, ImportSchoolNames, InviteUsers,
    RemoveUserFromSchool, SearchSchools, SetInternalSchoolName, UpdateSchoolName, VerifySchool,
};
use shared::domain::admin::{
    InviteFailedReason, InviteSchoolUserFailure, InviteSchoolUsersResponse, SearchSchoolsResponse,
};
use shared::domain::billing::{
    SchoolId, SchoolNameId, SchoolNameValue, SubscriptionStatus, UpdateSubscriptionPlansRequest,
};
use shared::error::AccountError;
use shared::{
    api::{
        endpoints::admin::{self, CreateUpdateSubscriptionPlans},
        ApiEndpoint, PathParts,
    },
    domain::{
        admin::{ExportDataRequest, ExportType},
        session::NewSessionResponse,
        user::UserId,
    },
    error::IntoAnyhow,
};
use sqlx::PgPool;
use tracing::instrument;

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

async fn create_or_update_subscription_plans(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    req: Json<<CreateUpdateSubscriptionPlans as ApiEndpoint>::Req>,
) -> actix_web::Result<HttpResponse, error::Server> {
    let UpdateSubscriptionPlansRequest { plans } = req.into_inner();

    for (plan_type, price_id) in plans {
        db::billing::upsert_subscription_plan(&db, plan_type, price_id).await?;
    }

    Ok(HttpResponse::Created().finish())
}

async fn search_schools(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Query(search): Query<<SearchSchools as ApiEndpoint>::Req>,
) -> Result<(Json<<SearchSchools as ApiEndpoint>::Res>, http::StatusCode), error::Server> {
    let (schools, schools_count) = try_join!(
        db::account::find_schools(db.as_ref(), &search),
        db::account::find_schools_count(db.as_ref(), &search)
    )?;

    let schools = schools.into_iter().map(From::from).collect();

    Ok((
        Json(SearchSchoolsResponse {
            schools,
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
        db::account::add_school_name(pool, name.into()).await?;
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

#[instrument(skip_all)]
async fn update_school_name(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    path: Path<SchoolNameId>,
    req: Json<<UpdateSchoolName as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <UpdateSchoolName as ApiEndpoint>::Err> {
    let school_name_id = path.into_inner();

    let new_name: SchoolNameValue = req.into_inner();

    if db::account::check_renamed_school_name_exists(
        db.as_ref(),
        new_name.as_ref(),
        &school_name_id,
    )
    .await
    .into_anyhow()?
    {
        return Err(AccountError::SchoolNameExists(new_name));
    }

    db::account::update_school_name(db.as_ref(), &school_name_id, new_name)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::Ok().finish())
}

#[instrument(skip_all)]
async fn create_school_name(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    req: Json<<CreateSchoolName as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<CreateSchoolName as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    <CreateSchoolName as ApiEndpoint>::Err,
> {
    let school_name: SchoolNameValue = req.into_inner();

    if db::account::check_school_name_exists(db.as_ref(), school_name.as_ref())
        .await
        .into_anyhow()?
    {
        return Err(AccountError::SchoolNameExists(school_name));
    }

    let school_name_id = db::account::add_school_name(db.as_ref(), school_name.into())
        .await
        .into_anyhow()?;

    Ok((Json(school_name_id), http::StatusCode::OK))
}

async fn verify_school(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    Json(data): Json<<VerifySchool as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Server> {
    db::account::verify_school(db.as_ref(), data.school_id, data.verified).await?;

    Ok(HttpResponse::Ok().finish())
}

async fn set_internal_school_name(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    path: Path<SchoolId>,
    Json(school_name_id): Json<<SetInternalSchoolName as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <SetInternalSchoolName as ApiEndpoint>::Err> {
    let school_id = path.into_inner();
    db::account::set_internal_school_name(db.as_ref(), school_id, school_name_id)
        .await
        .into_anyhow()?;

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
                    None => {
                        match account_summary.subscription_status {
                            Some(SubscriptionStatus::Expired) | None => {
                                // If they have an account with an expired subscription or no subscription,
                                // then delete that account so that they can be added to a school account.
                                db::account::delete_account_for_user(pool, &user_id).await?;
                                Ok(user_id)
                            }
                            _ => Err(InviteFailedReason::HasIndividualAccount),
                        }
                    }
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
            let school = db::account::get_school_account_by_id(pool, school_id)
                .await?
                .ok_or(anyhow!("School not found"))?;
            db::account::associate_user_with_account(
                pool,
                &user_id,
                &school.account_id,
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

async fn get_school_names(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
) -> Result<
    (Json<<GetSchoolNames as ApiEndpoint>::Res>, http::StatusCode),
    <GetSchoolNames as ApiEndpoint>::Err,
> {
    Ok((
        Json(
            db::account::get_unused_school_names(db.as_ref())
                .await
                .into_anyhow()?,
        ),
        http::StatusCode::OK,
    ))
}

async fn delete_user_account(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    user_id: Path<UserId>,
) -> Result<HttpResponse, <DeleteUserAccount as ApiEndpoint>::Err> {
    let user_id: UserId = user_id.into_inner();

    if let Some(account_summary) = db::account::get_user_account_summary(db.as_ref(), &user_id)
        .await
        .into_anyhow()?
    {
        match account_summary.subscription_status {
            Some(SubscriptionStatus::Expired) | None => {
                // Individual account users are always admin.
                // If they are an admin and...
                if account_summary.is_admin {
                    // If they have an account with an expired subscription or no subscription,
                    // then delete that account so that they can be added to a school account.
                    db::account::delete_account_for_user(db.as_ref(), &user_id).await?;
                    Ok(())
                } else {
                    Err(AccountError::BadRequest("User is not an admin".into()))
                }
            }
            _ => Err(AccountError::BadRequest(
                "User has an active subscription".into(),
            )),
        }
    } else {
        Ok(())
    }?;

    Ok(HttpResponse::Ok().finish())
}

async fn remove_user_from_school(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    school_id: Path<SchoolId>,
    req: Json<<RemoveUserFromSchool as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <DeleteUserAccount as ApiEndpoint>::Err> {
    let school_id = school_id.into_inner();
    let user_id: UserId = req.into_inner();

    db::account::delete_user_from_school(db.as_ref(), &school_id, &user_id)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::Ok().finish())
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
        <CreateUpdateSubscriptionPlans as ApiEndpoint>::Path::PATH,
        admin::CreateUpdateSubscriptionPlans::METHOD
            .route()
            .to(create_or_update_subscription_plans),
    )
    .route(
        <SearchSchools as ApiEndpoint>::Path::PATH,
        admin::SearchSchools::METHOD.route().to(search_schools),
    )
    .route(
        <ImportSchoolNames as ApiEndpoint>::Path::PATH,
        ImportSchoolNames::METHOD.route().to(import_school_names),
    )
    .route(
        <UpdateSchoolName as ApiEndpoint>::Path::PATH,
        UpdateSchoolName::METHOD.route().to(update_school_name),
    )
    .route(
        <CreateSchoolName as ApiEndpoint>::Path::PATH,
        CreateSchoolName::METHOD.route().to(create_school_name),
    )
    .route(
        <VerifySchool as ApiEndpoint>::Path::PATH,
        VerifySchool::METHOD.route().to(verify_school),
    )
    .route(
        <SetInternalSchoolName as ApiEndpoint>::Path::PATH,
        SetInternalSchoolName::METHOD
            .route()
            .to(set_internal_school_name),
    )
    .route(
        <InviteUsers as ApiEndpoint>::Path::PATH,
        InviteUsers::METHOD.route().to(invite_school_users),
    )
    .route(
        <GetSchoolNames as ApiEndpoint>::Path::PATH,
        GetSchoolNames::METHOD.route().to(get_school_names),
    )
    .route(
        <DeleteUserAccount as ApiEndpoint>::Path::PATH,
        DeleteUserAccount::METHOD.route().to(delete_user_account),
    )
    .route(
        <RemoveUserFromSchool as ApiEndpoint>::Path::PATH,
        RemoveUserFromSchool::METHOD
            .route()
            .to(remove_user_from_school),
    );
}
