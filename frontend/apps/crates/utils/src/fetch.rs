/*
 
   the _status functions return the status without doing side effects
   the non_status versions do side effects based on the status (e.g. redirect to no-auth page)
*/

use wasm_bindgen::prelude::*;
use shared::api::{
    method::Method,
    result::{HttpStatus, ResultResponse}
};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::JsFuture;
use shared::domain::auth::CSRF_HEADER_NAME;
use crate::{
    storage::load_csrf_token,
    unwrap::UnwrapJiExt,
    env::env_var
};
use js_sys::Promise;
use wasm_bindgen::JsCast;
use awsm_web::loaders::fetch::{fetch_with_headers_and_data, fetch_upload_file, fetch_with_data , fetch_upload_blob_with_headers, fetch_upload_file_with_headers};
use web_sys::{File, Blob};
use super::settings::SETTINGS;

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

//TODO - resumeable uploads
//https://cloud.google.com/storage/docs/performing-resumable-uploads#resume-upload
pub async fn upload_file_gcs(url:&str, file:&File) -> Result<(), ()> {
    let (resp, status) = upload_file_gcs_status(url, file).await;

    side_effect_error(status);

    resp
}

pub async fn upload_file_gcs_status(url:&str, file:&File) -> (Result<(), ()>, u16) {
    upload_file_direct_status(url, file, Method::Put).await
}

pub async fn upload_file_direct(url:&str, file:&File, method:Method) -> Result<(), ()> {
    let (resp, status) = upload_file_direct_status(url, file, method).await;

    side_effect_error(status);

    resp
}

pub async fn upload_file_direct_status(url:&str, file:&File, method:Method) -> (Result<(), ()>, u16) {
    let res = fetch_upload_file(&url, file, method.as_str()).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(()), status)
    }
}

pub async fn api_upload_blob(endpoint:&str, blob:&Blob, method:Method) -> Result<(), ()> {
    let (resp, status) = api_upload_blob_status(endpoint, blob, method).await;

    side_effect_error(status);

    resp

}

pub async fn api_upload_blob_status(endpoint:&str, blob:&Blob, method:Method) -> (Result<(), ()>, u16) {

    let (url, _) = api_get_query::<()>(endpoint, method, None);

    let csrf = load_csrf_token().unwrap_or_default();
    
    let res = fetch_upload_blob_with_headers(&url, blob, method.as_str(), true,&vec![(CSRF_HEADER_NAME, &csrf)]).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(()), status)
    }
}


pub async fn api_upload_file(endpoint:&str, file:&File, method:Method) -> Result<(), ()> {
    let (resp, status) = api_upload_file_status(endpoint, file, method).await;

    side_effect_error(status);

    resp

}

pub async fn api_upload_file_status(endpoint:&str, file:&File, method:Method) -> (Result<(), ()>, u16) {

    let (url, _) = api_get_query::<()>(endpoint, method, None);

    let csrf = load_csrf_token().unwrap_or_default();
    let res = fetch_upload_file_with_headers(&url, file, method.as_str(), true,&vec![(CSRF_HEADER_NAME, &csrf)]).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(()), status)
    }
}

pub async fn api_no_auth<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize {
    let (resp, status) = api_no_auth_status(endpoint, method, data).await;

    side_effect_error(status);

    resp 
}

pub async fn api_no_auth_status<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> (Result<T, E>, u16)
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
    }
}

//used in cases where we have the cookie but not the token
//really just used for login and registration, to get the token via oauth flow
pub async fn api_no_auth_with_credentials<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize 
{
    let (resp, status) = api_no_auth_with_credentials_status(endpoint, method, data).await;

    side_effect_error(status);

    resp
}

pub async fn api_no_auth_with_credentials_status<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> (Result<T, E>, u16)
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), true, data).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
    }
}

pub async fn api_with_token<T, E, Payload>(endpoint: &str, token:&str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let (resp, status) = api_with_token_status(endpoint, token, method, data).await;
    
    side_effect_error(status);

    resp
}

pub async fn api_with_token_status<T, E, Payload>(endpoint: &str, token:&str, method:Method, data:Option<Payload>) -> (Result<T, E>, u16)
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let bearer = format!("Bearer {}", token);

    let (url, data) = api_get_query(endpoint, method, data);
 
    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![("Authorization", &bearer)], data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
    }
}
//TODO - get rid of this, use specialization
pub async fn api_with_token_empty<E, Payload>(endpoint: &str, token: &str, method:Method, data:Option<Payload>) -> Result<(), E> 
where E: DeserializeOwned + Serialize, Payload: Serialize
{
    let (resp, status) = api_with_token_empty_status(endpoint, token, method, data).await;

    side_effect_error(status);

    resp
}
pub async fn api_with_token_empty_status<E, Payload>(endpoint: &str, token: &str, method:Method, data:Option<Payload>) -> (Result<(), E> , u16)
where E: DeserializeOwned + Serialize, Payload: Serialize
{
    let bearer = format!("Bearer {}", token);

    let (url, data) = api_get_query(endpoint, method, data);
 
    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![("Authorization", &bearer)], data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
    }
}

pub async fn api_with_auth<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let (resp, status) = api_with_auth_status(endpoint, method, data).await;
    
    side_effect_error(status);

    resp
}

pub async fn api_with_auth_status<T, E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> (Result<T, E>, u16)
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{

    if let Ok(token) = env_var("LOCAL_API_AUTH_OVERRIDE") {
        api_with_token_status(endpoint, &token, method, data).await
    } else {
        let csrf = load_csrf_token().unwrap_or_default();

        let (url, data) = api_get_query(endpoint, method, data);

        let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![(CSRF_HEADER_NAME, &csrf)], data)
            .await
            .unwrap_ji();


        let status = res.status();

        if res.ok() {
            (Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)), status)
        } else {
            (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
        }
    }
}
//TODO - get rid of this, use specialization
pub async fn api_with_auth_empty<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<(), E> 
where E: DeserializeOwned + Serialize, Payload: Serialize
{
    let (resp, status) = api_with_auth_empty_status(endpoint, method, data).await;

    side_effect_error(status);

    resp
}
pub async fn api_with_auth_empty_status<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> (Result<(), E> , u16)
where E: DeserializeOwned + Serialize, Payload: Serialize
{
    if let Ok(token) = env_var("LOCAL_API_AUTH_OVERRIDE") {
        api_with_token_empty_status(endpoint, &token, method, data).await
    } else {
        let csrf = load_csrf_token().unwrap_or_default();
        
        let (url, data) = api_get_query(endpoint, method, data);

        let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![(CSRF_HEADER_NAME, &csrf)], data)
            .await
            .unwrap_ji();


        let status = res.status();

        if res.ok() {
            (Ok(()), status)
        } else {
            (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
        }
    }
}

//TODO - get rid of this, use specialization
pub async fn api_no_auth_empty<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> Result<(), E> 
where E: DeserializeOwned + Serialize, Payload: Serialize {
    let (resp, status) = api_no_auth_empty_status(endpoint, method, data).await;

    side_effect_error(status);

    resp
}

//TODO - get rid of this, use specialization
pub async fn api_no_auth_empty_status<E, Payload>(endpoint: &str, method:Method, data:Option<Payload>) -> (Result<(), E>, u16)
where E: DeserializeOwned + Serialize, Payload: Serialize {


    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data).await.unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
    }
}


//really just used with login - see https://datatracker.ietf.org/doc/html/rfc7617#section-2
pub async fn api_with_basic_token<T, E, Payload>(endpoint: &str, user_id:&str, password:&str, method:Method, data:Option<Payload>) -> Result<T, E> 
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{
    let (resp, status) = api_with_basic_token_status(endpoint, user_id, password, method, data).await;

    side_effect_error(status);

    resp
}

pub async fn api_with_basic_token_status<T, E, Payload>(endpoint: &str, user_id:&str, password:&str, method:Method, data:Option<Payload>) -> (Result<T, E>, u16)
where T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize
{

    let credentials = format!("{}:{}", user_id, password); 
    let token = base64::encode(credentials.as_bytes());
    let basic = format!("Basic {}", token);

    let (url, data) = api_get_query(endpoint, method, data);
 
    let res = fetch_with_headers_and_data(&url, method.as_str(), true, &vec![("Authorization", &basic)], data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)), status)
    } else {
        (Err(res.json_from_str().await.expect_ji(DESERIALIZE_ERR)), status)
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

