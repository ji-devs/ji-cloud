use serde::{de::DeserializeOwned, Serialize};
use shared::api::result::ResultResponse;
use warp::reject::Rejection;

pub trait ReplyExt<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> {
    fn warp_reply(self) -> Result<warp::reply::Json, warp::reject::Rejection>;
}

pub type HandlerResult<T, E> = Result<Result<T, E>, Rejection>;

impl<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> ReplyExt<T, E>
    for HandlerResult<T, E>
{
    fn warp_reply(self) -> Result<warp::reply::Json, warp::reject::Rejection> {
        self.and_then(|ok| ok.warp_reply())
    }
}

impl<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> ReplyExt<T, E>
    for Result<T, E>
{
    fn warp_reply(self) -> Result<warp::reply::Json, warp::reject::Rejection> {
        match self {
            Self::Ok(value) => Ok(warp::reply::json(&ResultResponse::Ok::<T, E>(value))),
            Self::Err(value) => Ok(warp::reply::json(&ResultResponse::Err::<T, E>(value))),
        }
    }
}

/*
pub fn reply_empty_ok() -> Result<warp::reply::Json, warp::reject::Rejection> {
    reply_ok(())
}

pub fn reply_empty_err() -> Result<warp::reply::Json, warp::reject::Rejection> {
    reply_err(())
}
*/
