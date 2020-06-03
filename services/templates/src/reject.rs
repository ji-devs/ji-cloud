/*
    Generally speaking, requests should complete and use the data structure
    of the returned json to convey information. 
    
    A rejection is a harsh top-level cutoff

    The exception to that rule is things having to do with auth
    These errors do reject with http status codes - even if it's part of the account creation process
*/

use std::sync::Arc;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::convert::Infallible;
use ji_cloud_shared::response::HttpStatus;
use handlebars::Handlebars;
use crate::pages::direct::{DirectPage, direct_template};

use warp:: {
    http::{
        Method,
        StatusCode,
    },
    reject::Reject,
    Reply,
    Rejection,
    Filter,
};

pub trait CustomWarpRejection: Reject + Default {
    fn rejection() -> Rejection {
        warp::reject::custom(Self::default())
    }
}
impl <T: Reject + Default> CustomWarpRejection for T{}

#[derive(Debug, Default)]
pub struct PgPoolError;
impl Reject for PgPoolError {}

#[derive(Debug, Default)]
pub struct DbQueryError;
impl Reject for DbQueryError{}

#[derive(Debug, Default)]
pub struct NoAuth;
impl Reject for NoAuth {}

#[derive(Debug, Default)]
pub struct InternalError;
impl Reject for InternalError{}

#[derive(Debug, Default)]
pub struct RequiredData;
impl Reject for RequiredData{}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
/// An API error serializable to JSON.
pub async fn handle_rejection(hb:Arc<Handlebars<'_>>, err: Rejection) -> Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        return Ok(Box::new(direct_template(hb.clone(), DirectPage::NotFound).await.unwrap()));
    } else if let Some(NoAuth) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "UNAUTHORIZED";
    } else if let Some(InternalError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL ERROR";
    } else if let Some(PgPoolError) = err.find() {
        code = StatusCode::SERVICE_UNAVAILABLE;
        message = "DATABASE POOL ERROR";
    } else if let Some(DbQueryError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "DB QUERY ERROR";
    } else if let Some(RequiredData) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "REQUIRED DATA";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&HttpStatus {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(Box::new(warp::reply::with_status(json, code)))
}
