/*
    There are a few top-level rejections (esp auth-related)
    Everything else is not a rejection, rather it's always resolved (as ResultResponse)

    ResultResponse is itself divided into Ok/Err - but these are *expected* and recoverable errors
*/

use wasm_bindgen::prelude::*;
use shared::api::{
    method::Method,
    result::{HttpStatus, ResultResponse}
};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::JsFuture;
use shared::domain::auth::CSRF_HEADER_NAME;
use crate::storage::load_csrf_token; 
use js_sys::Promise;
use wasm_bindgen::JsCast;
use awsm_web::loaders::fetch::{fetch_with_headers_and_data, fetch_with_data , fetch_upload_blob_with_headers, fetch_upload_file_with_headers};
use web_sys::{File, Blob};
use super::settings::SETTINGS;
use crate::unwrap::UnwrapJiExt;

pub use awsm_web::loaders::helpers::{spawn_handle, FutureHandle};


#[derive(Debug)]
pub enum Error {
    AuthForbidden,
    AuthCompleteRegistration,
    HttpStatusCodeOnly(u16),
    HttpStatus(HttpStatus),
    JsValue(JsValue),
}

pub const POST:&'static str = "POST";
pub const GET:&'static str = "GET";

const DESERIALIZE_ERR:&'static str = "couldn't deserialize error in fetch";
const DESERIALIZE_OK:&'static str = "couldn't deserialize ok in fetch";

//either a serialized error or a native error (like 401, 403, etc.)


fn api_get_query<'a, T: Serialize>(endpoint:&'a str, method:Method, data: Option<T>) -> (String, Option<T>) {

    let api_url = SETTINGS.get().unwrap_ji().remote_target.api_url();

    if method == Method::Get {
        if let Some(data) = data {
            let query = serde_qs::to_string(&data).unwrap_ji();
            let url = format!("{}{}?{}", api_url, endpoint, query);
            (url, None)
        } else {
            let url = format!("{}{}", api_url, endpoint);
            (url, None)
        }
    } else {
        let url = format!("{}{}", api_url, endpoint);
        (url, data)
    }
}

pub async fn api_upload_blob(endpoint:&str, blob:&Blob, method:Method) -> Result<(), ()> {

    let (url, _) = api_get_query::<()>(endpoint, method, None);

    let csrf = load_csrf_token().unwrap_ji();

    let res = fetch_upload_blob_with_headers(&url, blob, method.as_str(), true,&vec![(CSRF_HEADER_NAME, &csrf)]).await.unwrap_ji();
    if res.ok() {
        Ok(())
    } else {
        side_effect_error(res.status());
        Err(())
    }
}
pub async fn api_upload_file(endpoint:&str, file:&File, method:Method) -> Result<(), ()> {

    let (url, _) = api_get_query::<()>(endpoint, method, None);

    let csrf = load_csrf_token().unwrap_ji();

    let res = fetch_upload_file_with_headers(&url, file, method.as_str(), true,&vec![(CSRF_HEADER_NAME, &csrf)]).await.unwrap_ji();
    if res.ok() {
        Ok(())
    } else {
        side_effect_error(res.status());
        Err(())
    }
}

pub async fn api_no_auth<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data).await.unwrap_ji();

    if res.ok() {
        Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK))
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

//really just used for login and registration, but w/e
pub async fn api_no_auth_with_credentials<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), true, data).await.unwrap_ji();

    if res.ok() {
        Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK))
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

pub async fn api_with_token<T, E, Payload>(endpoint: &str, token:&str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let bearer = format!("Bearer {}", token);

    let (url, data) = api_get_query(endpoint, method, data);
 
    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![("Authorization", &bearer)], data)
        .await
        .unwrap_ji();
    if res.ok() {
        Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK))
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

pub async fn api_with_auth<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let csrf = load_csrf_token().expect_ji("no CSRF / not logged in!");

    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![(CSRF_HEADER_NAME, &csrf)], data)
        .await
        .unwrap_ji();


    if res.ok() {
        Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK))
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

//TODO - get rid of this, use specialization
pub async fn api_no_auth_empty<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<(), E> 
where E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data).await.unwrap_ji();

    if res.ok() {
        Ok(())
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

//TODO - get rid of this, use specialization
pub async fn api_with_auth_empty<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<(), E> 
where E: DeserializeOwned + Serialize, Payload: Serialize
{
    let csrf = load_csrf_token().unwrap_ji();
    
    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![(CSRF_HEADER_NAME, &csrf)], data)
        .await
        .unwrap_ji();
    if res.ok() {
        Ok(())
    } else {
        side_effect_error(res.status());
        Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))
    }
}

fn side_effect_error(status_code:u16) -> bool {
    match status_code {
        403 | 401 => {
            web_sys::window().unwrap_ji().alert_with_message(crate::strings::STR_AUTH_ALERT);
            true
        },
        _ => false
    }
} 

