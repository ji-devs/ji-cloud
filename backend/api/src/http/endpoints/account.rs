use crate::extractor::TokenUser;
use crate::{db, error};
use actix_web::web::{Data, Json, ServiceConfig};
use shared::api::endpoints::account::GetSchoolNames;
use shared::api::{endpoints::account::CreateSchoolAccount, ApiEndpoint, PathParts};
use shared::domain::billing::{CreateSchoolAccountRequest, SchoolNameRequest};
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

    let school_name_id = match req.name {
        SchoolNameRequest::Value(new_name) => {
            // If the user is creating a school with a new school name that we don't already
            // know about, then check whether that name already exists
            if db::account::check_school_name_exists(db.as_ref(), &new_name).await? {
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
            db::account::create_default_school_account(
                db.as_ref(),
                auth.user_id(),
                school_name_id,
                req.location,
            )
            .await?,
        ),
        http::StatusCode::CREATED,
    ))
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
    );
}
