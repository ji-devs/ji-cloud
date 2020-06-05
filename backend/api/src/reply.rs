use std::convert::Infallible;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use ji_cloud_shared::response::ResultResponse;



pub fn reply_ok<T: Serialize + DeserializeOwned>(value:T) -> Result<warp::reply::Json, warp::reject::Rejection> {
    Ok(warp::reply::json(&ResultResponse::Ok::<T, ()>(value)))
}
pub fn reply_empty_ok() -> Result<warp::reply::Json, warp::reject::Rejection> {
    reply_ok(())
}

pub fn reply_err<T: Serialize + DeserializeOwned>(value:T) -> Result<warp::reply::Json, warp::reject::Rejection> {
    Ok(warp::reply::json(&ResultResponse::Err::<(), T>(value)))
}

pub fn reply_empty_err() -> Result<warp::reply::Json, warp::reject::Rejection> {
    reply_err(())
}
