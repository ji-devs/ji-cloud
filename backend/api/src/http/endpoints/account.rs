use crate::db::account::AccountMember;
use crate::extractor::TokenUser;
use crate::{db, error};
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::HttpResponse;
use futures::try_join;
use shared::api::endpoints::account::{
    DeleteSchoolAccount, GetSchoolAccount, GetSchoolNames, UpdateSchoolAccount, UpdateSchoolName,
};
use shared::api::{endpoints::account::CreateSchoolAccount, ApiEndpoint, PathParts};
use shared::domain::billing::{
    AccountId, AccountIfAuthorized, CreateSchoolAccountRequest, GetSchoolAccountResponse, SchoolId,
    SchoolNameRequest, SchoolNameValue, UpdateSchoolAccountRequest,
};
use shared::domain::user::{UserId, UserScope};
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
    error::Account,
> {
    if db::account::check_user_has_account(db.as_ref(), auth.user_id()).await? {
        return Err(error::Account::UserHasAccount);
    }

    let req: CreateSchoolAccountRequest = req.into_inner();

    let school_name_id = match req.name.clone() {
        SchoolNameRequest::Value(new_name) => {
            // If the user is creating a school with a new school name that we don't already
            // know about, then check whether that name already exists
            if db::account::check_school_name_exists(db.as_ref(), new_name.as_ref()).await? {
                // Otherwise, return an error to the client
                return Err(error::Account::SchoolNameExists(new_name));
            }

            // If it doesnt exist, then add the name
            db::account::add_school_name(db.as_ref(), new_name, false).await?
        }
        SchoolNameRequest::Id(id) => {
            // If they are creating a school with an existing school name, then check that
            // another school doesn't already exist that uses the same name.
            if db::account::check_school_exists(db.as_ref(), &id).await? {
                // If one exists, return an error to the client.
                return Err(error::Account::SchoolExists(id));
            }

            id
        }
    };
    // If no school exists, then create the school with school name ID. and using the
    // currently logged in user as the admin and their email as the schools contact email.
    Ok((
        Json(
            db::account::create_school_account(db.as_ref(), auth.user_id(), &school_name_id, req)
                .await?,
        ),
        http::StatusCode::CREATED,
    ))
}

#[instrument(skip_all)]
async fn update_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
    req: Json<<UpdateSchoolAccount as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Account> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await?
        .ok_or(error::Account::NotFound("School not found".into()))?;

    user_authorization(db.as_ref(), &user_id, &school.account_id)
        .await?
        .test_authorized(true)?;

    let req: UpdateSchoolAccountRequest = req.into_inner();

    db::account::update_school_account(db.as_ref(), &school_id, req.into()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[instrument(skip_all)]
async fn update_school_name(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
    req: Json<<UpdateSchoolName as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Account> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let new_name: SchoolNameValue = req.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await?
        .ok_or(error::Account::NotFound("School not found".into()))?;

    let authorization = user_authorization(db.as_ref(), &user_id, &school.account_id).await?;
    authorization.test_authorized(true)?;

    if db::account::check_renamed_school_name_exists(db.as_ref(), new_name.as_ref(), &school_id)
        .await?
    {
        return Err(error::Account::SchoolNameExists(new_name));
    }

    // If the user is a system administrator then the verified flag is automatically set to true.
    // Otherwise it's false for all other users.
    db::account::update_school_name(
        db.as_ref(),
        &school.school_name.id,
        new_name,
        authorization.is_system_administrator(),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[instrument(skip_all)]
async fn get_school_names(
    _auth: TokenUser,
    db: Data<PgPool>,
) -> Result<Json<<GetSchoolNames as ApiEndpoint>::Res>, error::Account> {
    Ok(Json(
        db::account::get_verified_school_names(db.as_ref()).await?,
    ))
}

async fn get_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
) -> Result<Json<<GetSchoolAccount as ApiEndpoint>::Res>, error::Account> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await?
        .ok_or(error::Account::NotFound("School not found".into()))?;

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

async fn delete_school_account(
    auth: TokenUser,
    db: Data<PgPool>,
    path: Path<SchoolId>,
) -> Result<HttpResponse, error::Account> {
    let user_id = auth.user_id();
    let school_id = path.into_inner();

    let school = db::account::get_school_account_by_id(db.as_ref(), &school_id)
        .await?
        .ok_or(error::Account::NotFound("School not found".into()))?;

    let authorization = user_authorization(db.as_ref(), &user_id, &school.account_id).await?;

    authorization.test_authorized(true)?;

    let (account, users) = try_join!(
        db::account::get_account_by_id(db.as_ref(), &school.account_id),
        db::account::get_account_users_by_account_id(db.as_ref(), &school.account_id),
    )?;
    let account = account.ok_or(anyhow::anyhow!("School {} account is missing", school.id))?;

    if let Some(subscription) = account.subscription {
        if subscription.status.is_valid() {
            return Err(error::Account::Forbidden);
        }
    }

    match authorization {
        UserAuthorization::AccountAdministrator => {
            // If the current user is an account admin and they're the only member of this school,
            // then the school account can be deleted.
            if users.len() > 1 {
                return Err(error::Account::Forbidden);
            }
        }
        UserAuthorization::SystemAdministrator => {
            // System admins can only delete an account once no users are associated or they're the admin and only user
            if !users.is_empty() {
                if users.len() == 1 {
                    let user = users.first().unwrap();
                    if user.user.id != user_id {
                        // Cannot delete if the associated user is not the current user
                        return Err(error::Account::Forbidden);
                    }
                } else {
                    // Multiple users
                    return Err(error::Account::Forbidden);
                }
            }
        }
        _ => return Err(error::Account::Forbidden),
    }

    db::account::delete_school_account(db.as_ref(), &school.account_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
enum UserAuthorization {
    SystemAdministrator,
    AccountAdministrator,
    AccountMember,
}

impl UserAuthorization {
    fn is_system_administrator(&self) -> bool {
        matches!(self, UserAuthorization::SystemAdministrator)
    }

    fn test_authorized(&self, require_account_admin: bool) -> Result<(), error::Account> {
        if self.is_authorized(require_account_admin) {
            Ok(())
        } else {
            Err(error::Account::Forbidden)
        }
    }

    fn is_authorized(&self, require_account_admin: bool) -> bool {
        match self {
            UserAuthorization::AccountMember if require_account_admin => false,
            _ => true,
        }
    }
}

async fn user_authorization(
    db: &PgPool,
    user_id: &UserId,
    account_id: &AccountId,
) -> Result<UserAuthorization, error::Account> {
    Ok(
        if db::user::has_scopes(db, *user_id, &[UserScope::Admin]).await? {
            UserAuthorization::SystemAdministrator
        } else {
            match db::account::user_account_membership(db, user_id, account_id).await? {
                Some(AccountMember::Admin) => UserAuthorization::AccountAdministrator,
                Some(AccountMember::User) => UserAuthorization::AccountMember,
                None => return Err(error::Account::Forbidden),
            }
        },
    )
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <CreateSchoolAccount as ApiEndpoint>::Path::PATH,
        CreateSchoolAccount::METHOD
            .route()
            .to(create_school_account),
    )
    .route(
        <GetSchoolNames as ApiEndpoint>::Path::PATH,
        GetSchoolNames::METHOD.route().to(get_school_names),
    )
    .route(
        <GetSchoolAccount as ApiEndpoint>::Path::PATH,
        GetSchoolAccount::METHOD.route().to(get_school_account),
    )
    .route(
        <UpdateSchoolAccount as ApiEndpoint>::Path::PATH,
        UpdateSchoolAccount::METHOD
            .route()
            .to(update_school_account),
    )
    .route(
        <UpdateSchoolName as ApiEndpoint>::Path::PATH,
        UpdateSchoolName::METHOD.route().to(update_school_name),
    )
    .route(
        <DeleteSchoolAccount as ApiEndpoint>::Path::PATH,
        DeleteSchoolAccount::METHOD
            .route()
            .to(delete_school_account),
    );
}
