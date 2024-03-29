use crate::db;
use crate::domain::{user_authorization, UserAuthorization};
use crate::extractor::{ScopeAdmin, TokenUser, TokenUserWithScope};
use crate::stripe::create_stripe_client;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::HttpResponse;
use anyhow::anyhow;
use futures::try_join;
use ji_core::settings::RuntimeSettings;
use shared::api::endpoints::account::{
    DeleteSchoolAccount, GetIndividualAccount, GetSchoolAccount, UpdateSchoolAccount,
};
use shared::api::endpoints::admin::{GetAdminSchoolAccount, SetAccountTierOverride};
use shared::api::{endpoints::account::CreateSchoolAccount, ApiEndpoint, PathParts};
use shared::domain::admin::GetAdminSchoolAccountResponse;
use shared::domain::billing::{
    AccountIfAuthorized, CreateSchoolAccountRequest, GetSchoolAccountResponse,
    IndividualAccountResponse, SchoolId, SubscriptionStatus, UpdateSchoolAccountRequest,
    UserAccountSummary,
};
use shared::domain::user::UserId;
use shared::domain::UpdateNonNullable;
use shared::error::{AccountError, IntoAnyhow};
use sqlx::PgPool;
use tracing::instrument;

#[instrument(skip_all)]
async fn create_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    req: Json<<CreateSchoolAccount as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<CreateSchoolAccount as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    <CreateSchoolAccount as ApiEndpoint>::Err,
> {
    let user_id = auth.user_id();
    if db::account::check_user_has_account(db.as_ref(), user_id)
        .await
        .into_anyhow()?
    {
        if let Some(account_summary) = db::account::get_user_account_summary(db.as_ref(), &user_id)
            .await
            .into_anyhow()?
        {
            if account_summary.school_id.is_some() {
                return Err(AccountError::UserHasAccount);
            }

            // If they are an individual account with now subscription or an expired subscription,
            // then they can create a school subscription.
            match account_summary.subscription_status {
                Some(SubscriptionStatus::Expired) | None => {
                    // If they have an account with an expired subscription or no subscription,
                    // then delete that account so that they can be added to a school account.
                    db::account::delete_account_for_user(db.as_ref(), &user_id).await?;
                }
                _ => return Err(AccountError::UserHasAccount),
            }
        } else {
            return Err(AccountError::UserHasAccount);
        }
    }

    let req: CreateSchoolAccountRequest = req.into_inner();

    Ok((
        Json(
            db::account::create_school_account(db.as_ref(), auth.user_id(), req)
                .await
                .into_anyhow()?,
        ),
        http::StatusCode::CREATED,
    ))
}

#[instrument(skip_all)]
async fn update_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    path: Path<SchoolId>,
    req: Json<<UpdateSchoolAccount as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <UpdateSchoolAccount as ApiEndpoint>::Err> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await
        .into_anyhow()?
        .ok_or(AccountError::NotFound("School not found".into()))?;

    user_authorization(db.as_ref(), &user_id, &school.account_id)
        .await?
        .test_authorized(true)?;

    let req: UpdateSchoolAccountRequest = req.into_inner();

    let mut should_update_stripe = false;
    let mut update_customer = stripe::UpdateCustomer::default();

    if let UpdateNonNullable::Change(email) = &req.email {
        should_update_stripe = true;
        update_customer.email = Some(email.as_ref());
    }

    if let UpdateNonNullable::Change(school_name) = &req.school_name {
        should_update_stripe = true;
        update_customer.name = Some(school_name.as_ref());
    }

    if should_update_stripe {
        let client = create_stripe_client(&settings)?;
        let account_id = db::account::get_account_id_by_school_id(db.as_ref(), &school_id)
            .await?
            .ok_or(anyhow!("Missing account for school {school_id}"))?;

        let account = db::account::get_account_by_id(db.as_ref(), &account_id)
            .await?
            .ok_or(anyhow!("Missing account {account_id}"))?;

        let stripe_customer_id =
            stripe::CustomerId::from(account.stripe_customer_id.unwrap().clone());

        stripe::Customer::update(&client, &stripe_customer_id, update_customer).await?;
    }

    db::account::update_school_account(db.as_ref(), &school_id, req)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::Ok().finish())
}

async fn get_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
) -> Result<Json<<GetSchoolAccount as ApiEndpoint>::Res>, <GetSchoolAccount as ApiEndpoint>::Err> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await
        .into_anyhow()?
        .ok_or(AccountError::NotFound("School not found".into()))?;

    let authorization = user_authorization(db.as_ref(), &user_id, &school.account_id).await?;

    let (account, users) = try_join!(
        async {
            if authorization.is_authorized(true) {
                Ok(AccountIfAuthorized::Authorized(
                    db::account::get_account_by_id(db.as_ref(), &school.account_id)
                        .await?
                        .ok_or(anyhow::anyhow!("School {} account is missing", school.id))?,
                ))
            } else {
                Ok(AccountIfAuthorized::Unauthorized)
            }
        },
        db::account::get_account_users_by_account_id(db.as_ref(), &school.account_id),
    )?;

    Ok(Json(GetSchoolAccountResponse {
        school,
        account,
        users,
    }))
}

async fn get_admin_school_account(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    path: Path<SchoolId>,
) -> Result<
    Json<<GetAdminSchoolAccount as ApiEndpoint>::Res>,
    <GetAdminSchoolAccount as ApiEndpoint>::Err,
> {
    let school_id = path.into_inner();

    let school = db::account::get_admin_school_account_by_id(db.as_ref(), &school_id)
        .await
        .into_anyhow()?
        .ok_or(AccountError::NotFound("School not found".into()))?;

    let (account, users) = try_join!(
        async {
            Ok(
                db::account::get_account_by_id(db.as_ref(), &school.account_id)
                    .await?
                    .ok_or(anyhow::anyhow!("School {} account is missing", school.id))?,
            )
        },
        db::account::get_account_users_by_account_id(db.as_ref(), &school.account_id),
    )?;

    Ok(Json(GetAdminSchoolAccountResponse {
        school,
        account,
        users,
    }))
}

async fn set_account_tier_override(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
    path: Path<UserId>,
    req: Json<<SetAccountTierOverride as ApiEndpoint>::Req>,
) -> Result<HttpResponse, <SetAccountTierOverride as ApiEndpoint>::Err> {
    let user_id = path.into_inner();
    let tier_override = req.into_inner();

    let account_summary: UserAccountSummary =
        match db::account::get_user_account_summary(db.as_ref(), &user_id)
            .await
            .into_anyhow()?
        {
            Some(summary) => summary,
            None => {
                // Create a default user account if the summary is empty
                db::account::create_default_individual_account(db.as_ref(), &user_id)
                    .await
                    .into_anyhow()?;
                db::account::get_user_account_summary(db.as_ref(), &user_id)
                    .await
                    .into_anyhow()?
                    .ok_or(anyhow::anyhow!(
                        "User {} account summary is missing",
                        user_id
                    ))?
            }
        };

    let account_id = account_summary
        .account_id
        .ok_or(anyhow::anyhow!("Missing account_id"))?;

    db::account::set_account_tier_override(db.as_ref(), &account_id, tier_override)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::Ok().finish())
}

async fn delete_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
) -> Result<HttpResponse, <DeleteSchoolAccount as ApiEndpoint>::Err> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await
        .into_anyhow()?
        .ok_or(AccountError::NotFound("School not found".into()))?;

    let authorization = user_authorization(db.as_ref(), &user_id, &school.account_id).await?;

    authorization.test_authorized(true)?;

    let (account, users) = try_join!(
        db::account::get_account_by_id(db.as_ref(), &school.account_id),
        db::account::get_account_users_by_account_id(db.as_ref(), &school.account_id),
    )?;
    let account = account.ok_or(anyhow::anyhow!("School {} account is missing", school.id))?;

    if let Some(subscription) = account.subscription {
        if subscription.status.is_valid() {
            return Err(AccountError::Forbidden);
        }
    }

    match authorization {
        UserAuthorization::AccountAdministrator => {
            // If the current user is an account admin and they're the only member of this school,
            // then the school account can be deleted.
            if users.len() > 1 {
                return Err(AccountError::Forbidden);
            }
        }
        UserAuthorization::SystemAdministrator => {
            // System admins can only delete an account once no users are associated or they're the admin and only user
            if !users.is_empty() {
                if users.len() == 1 {
                    let user = users.first().unwrap();
                    if user.user.id != user_id {
                        // Cannot delete if the associated user is not the current user
                        return Err(AccountError::Forbidden);
                    }
                } else {
                    // Multiple users
                    return Err(AccountError::Forbidden);
                }
            }
        }
        _ => return Err(AccountError::Forbidden),
    }

    db::account::delete_school_account(db.as_ref(), &school.account_id)
        .await
        .into_anyhow()?;

    Ok(HttpResponse::Ok().finish())
}

async fn get_individual_account(
    auth: TokenUser,
    db: Data<PgPool>,
) -> Result<
    Json<<GetIndividualAccount as ApiEndpoint>::Res>,
    <GetIndividualAccount as ApiEndpoint>::Err,
> {
    let account = db::account::get_account_by_user_id(db.as_ref(), &auth.user_id()).await?;

    if let Some(account) = &account {
        if account.account_type.has_admin() {
            // We only want to return individual account details here.
            return Err(AccountError::Forbidden);
        }
    }

    Ok(Json(IndividualAccountResponse { account }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <CreateSchoolAccount as ApiEndpoint>::Path::PATH,
        CreateSchoolAccount::METHOD
            .route()
            .to(create_school_account),
    )
    .route(
        <GetSchoolAccount as ApiEndpoint>::Path::PATH,
        GetSchoolAccount::METHOD.route().to(get_school_account),
    )
    .route(
        <GetAdminSchoolAccount as ApiEndpoint>::Path::PATH,
        GetAdminSchoolAccount::METHOD
            .route()
            .to(get_admin_school_account),
    )
    .route(
        <SetAccountTierOverride as ApiEndpoint>::Path::PATH,
        SetAccountTierOverride::METHOD
            .route()
            .to(set_account_tier_override),
    )
    .route(
        <UpdateSchoolAccount as ApiEndpoint>::Path::PATH,
        UpdateSchoolAccount::METHOD
            .route()
            .to(update_school_account),
    )
    .route(
        <DeleteSchoolAccount as ApiEndpoint>::Path::PATH,
        DeleteSchoolAccount::METHOD
            .route()
            .to(delete_school_account),
    )
    .route(
        <GetIndividualAccount as ApiEndpoint>::Path::PATH,
        GetIndividualAccount::METHOD
            .route()
            .to(get_individual_account),
    );
}
