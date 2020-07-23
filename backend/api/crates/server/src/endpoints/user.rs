use actions::user::{get_profile, get_by_id, get_by_email, register};
use shared::{
    auth::{AuthClaims, RegisterRequest, RegisterError, SingleSignOnSuccess},
    user::NoSuchUserError,
    api::{
        result::ResultResponse,
        endpoints::{
            ApiEndpoint,
            user::{Profile, SingleSignOn},
        }
    }
};
use core::settings::SETTINGS;
use crate::{
    reply::HandlerResult,
    auth::reply_signin_auth,
    reject::{CustomWarpRejection, InternalError, DbQueryError}
};
use sqlx::postgres::PgPool;
use jsonwebtoken as jwt;

pub async fn handle_get_profile(claims:AuthClaims, db:PgPool) -> HandlerResult< <Profile as ApiEndpoint>::Res, <Profile as ApiEndpoint>::Err> {

    Ok(get_profile(&db, &claims.id).await)
}

//register handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_register(user_id:String, req:RegisterRequest, db:PgPool) -> Result<impl warp::Reply, warp::Rejection> {



    let err:Option<RegisterError> = {
        if get_by_id(&db, &user_id).await.is_some() {
            Some(RegisterError::TakenId)
        } else if req.display_name.is_empty() {
            Some(RegisterError::EmptyDisplayname)
        } else if req.first_name.is_empty() {
            Some(RegisterError::EmptyFirstname)
        } else if req.last_name.is_empty() {
            Some(RegisterError::EmptyLastname)
        } else if get_by_email(&db, &req.email).await.is_some() {
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

            register(&db, &user_id, &req)
                .await
                .map_err(|_| DbQueryError::rejection())?;

            reply_signin_auth(user_id, Vec::new(), true)
        }
    }
}

//the user_id is already validated in terms of firebase auth
//now we need to check with the database
//login handler doesn't use the usual wrapper since it needs to set the header
pub async fn handle_signin_credentials(user_id:String, db:PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    log::info!("Firebase is valid! user id is: {}", user_id);


    match get_by_id(&db, &user_id).await {
        Some(user) => reply_signin_auth(user_id, user.roles, false),
        None => {
            log::info!("hmm couldn't get user by id {}", user_id);
            //Since the happy path is a WithHeader reply, need wrap the sad path too
            let reply = warp::reply::json(&ResultResponse::Err::<(), NoSuchUserError>(NoSuchUserError{}));
            //TODO: https://github.com/seanmonstar/warp/issues/587#issuecomment-633961421
            //let reply = warp::reply::WithHeader { header: None, reply };

            //placeholder for now until we can really have empty WithHeader
            let reply = warp::reply::with_header(reply, "foo", "bar");
            Ok(reply)
        }
    }


}


//the claims are already validated from cookie and db lookup 
//no need to validate anything else
pub async fn handle_get_sso_jwt(auth:AuthClaims) -> HandlerResult< <SingleSignOn as ApiEndpoint>::Res, <SingleSignOn as ApiEndpoint>::Err> {

    log::info!("Firebase is valid! user id is: {}", auth.id);

    let claims = AuthClaims {
        id: auth.id,
        csrf: None,
        roles: auth.roles
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, &SETTINGS.get().unwrap().jwt_encoding_key).map_err(|_| InternalError::rejection())?;

    Ok(Ok(SingleSignOnSuccess{jwt}))
}
