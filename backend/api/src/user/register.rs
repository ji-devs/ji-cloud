use crate::reject::{CustomWarpRejection, DbQueryError, InternalError};
use ji_cloud_shared::{
    auth::{RegisterRequest, RegisterError, SigninSuccess},
    api::result::ResultResponse,
};
use crate::db::Db;
use diesel::prelude::*;
use diesel::insert_into;
use super::queries::{get_by_email, get_by_id};
use super::auth::reply_signin_auth;
use crate::db::{pg_pool, PgPool, get_db};
use crate::reply::HandlerResult;

//the user_id is already validated in terms of firebase auth

//register handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_register(user_id:String, req:RegisterRequest, pool:PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::users::dsl::*;

    log::info!("user id: {}", user_id);

    let db = get_db(pool)?;

    let err:Option<RegisterError> = {
        if get_by_id(&db, &user_id).is_some() {
            Some(RegisterError::TakenId)
        } else if req.display_name.is_empty() {
            Some(RegisterError::EmptyDisplayname)
        } else if req.first_name.is_empty() {
            Some(RegisterError::EmptyFirstname)
        } else if req.last_name.is_empty() {
            Some(RegisterError::EmptyLastname)
        } else if get_by_email(&db, &req.email).is_some() {
            Some(RegisterError::TakenEmail)
        } else {
            None
        }
    };

    match err {
        Some(err) => {
            //Since the happy path is a WithHeader reply, need wrap the sad path too
            let reply = warp::reply::json(&ResultResponse::Err::<(), RegisterError>(err));
            //TODO: https://github.com/seanmonstar/warp/issues/587#issuecomment-633961421
            //let reply = warp::reply::WithHeader { header: None, reply };

            //placeholder for now until we can really have empty WithHeader
            let reply = warp::reply::with_header(reply, "foo", "bar");
            Ok(reply)
        },
        None => {
            insert_into(users)
                .values((
                    id.eq(&user_id),
                    display_name.eq(req.display_name),
                    first_name.eq(req.first_name),
                    last_name.eq(req.last_name),
                    email.eq(req.email),
                ))
                .execute(&db)
                .map_err(|_| DbQueryError::rejection())?;
            
            reply_signin_auth(user_id, Vec::new(), true)
        }
    }
}
