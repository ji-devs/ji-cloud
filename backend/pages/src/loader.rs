use futures_util::future::TryFutureExt;
use serde::de::DeserializeOwned;
use crate::reject::{CustomWarpRejection, RequiredData};
use core::settings::SETTINGS;

pub async fn load_string(path:&str) -> Result<String, warp::Rejection> {
    reqwest::Client::new()
        .get(path)
        .send()
        .and_then(|res| res.text())
        .await
        .map_err(|err| RequiredData::rejection())
}

pub async fn load_json<T: DeserializeOwned>(path:&str) -> Result<T, warp::Rejection> {
    reqwest::Client::new()
        .get(path)
        .send()
        .and_then(|res| res.json())
        .await
        .map_err(|err| RequiredData::rejection())
}
