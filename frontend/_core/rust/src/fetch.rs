pub mod user;

/*
    There are a few top-level rejections (esp auth-related)
    Everything else is not a rejection, rather it's always resolved (as ResultResponse)

    ResultResponse is itself divided into Ok/Err - but these are *expected* and recoverable errors
*/

use wasm_bindgen::prelude::*;
use shared::api::result::{HttpStatus, ResultResponse};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::JsFuture;
use shared::auth::CSRF_HEADER_NAME;
use crate::storage::load_csrf_token; 
use js_sys::Promise;
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub enum Error {
    AuthForbidden,
    AuthCompleteRegistration,
    HttpStatusCodeOnly(u16),
    HttpStatus(HttpStatus),
    JsValue(JsValue),
}

// unwrap is the usual case, where anything other than a ResultResponse is a panic / dev error
// Therefore we can treat the ResultResponse itself as a Result
pub async fn api_with_auth_unwrap<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize>(url: &str, data:Option<Payload>) -> Result<T, E> {
    //inability to get the token from LocalStorage is almost definitely a programmer error, not a user error
    //the reason is that requests to these endpoints only happen after the user is signed in
    //if the session is _invalid_ then it will still be unwrapped though... 
    //that shouldn't happen unless the user has been inactive long enough for the cookie to
    //expire though so a hard failure is fine (they'll refresh their screen)
    //

    let csrf = load_csrf_token().unwrap_throw();
    
    let req = get_request_with_headers(&url, &vec![(CSRF_HEADER_NAME, &csrf)], data).map_err(|err| Error::JsValue(err)).unwrap_throw();

    api_fetch_json_req_unwrap(req).await
}
// api calls with token (in header)

pub async fn api_with_token_unwrap<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize>(url: &str, token:&str, data:Option<Payload>) -> Result<T, E> {

    let bearer = format!("Bearer {}", token);
    
    let req = get_request_with_headers(&url, &vec![("Authorization", &bearer)], data).map_err(|err| Error::JsValue(err)).unwrap_throw();

    api_fetch_json_req_unwrap(req).await
}


pub async fn api_fetch_json_unwrap<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize>(url: &str) -> Result<T, E> {
    let req = web_sys::Request::new_with_str(url).unwrap_throw();
    let res = api_fetch_json_req_unwrap(req).await?;
    Ok(res)
}

pub async fn api_fetch_json_req_unwrap<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize>(req: web_sys::Request) -> Result<T, E> {
    let resp: web_sys::Response = api_fetch_request(&req).await.unwrap_throw();

    let promise = resp.json().map_err(|err| Error::JsValue(err)).unwrap_throw();

    let data = JsFuture::from(promise).await.map_err(|err| Error::JsValue(err)).unwrap_throw();

    let resp:ResultResponse<T,E> = serde_wasm_bindgen::from_value(data).map_err(|err| Error::JsValue(err.into())).unwrap_throw();
    resp.into()
}


// The absolute case, where we want to deal with non-ResultResponse errors 
// api calls with auth (csrf in header, jwt in cookie)
pub async fn api_with_auth<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize>(url: &str, data:Option<Payload>) -> Result<ResultResponse<T, E>, Error> {
    let csrf = load_csrf_token().unwrap_throw();
    
    let req = get_request_with_headers(&url, &vec![(CSRF_HEADER_NAME, &csrf)], data).map_err(|err| Error::JsValue(err))?;
    api_fetch_json_req(req).await
}

pub async fn api_with_token<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize, Payload: Serialize>(url: &str, token:&str, data:Option<Payload>) -> Result<ResultResponse<T, E>, Error> {

    let bearer = format!("Bearer {}", token);
    
    let req = get_request_with_headers(&url, &vec![("Authorization", &bearer)], data).map_err(|err| Error::JsValue(err))?;

    api_fetch_json_req(req).await
}


// plain requests
pub async fn api_fetch_json<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize>(url: &str) -> Result<ResultResponse<T, E>, Error> {
    let req = web_sys::Request::new_with_str(url).unwrap_throw();
    let res = api_fetch_json_req(req).await?;
    Ok(res)
}

pub async fn api_fetch_json_req<T: DeserializeOwned + Serialize, E: DeserializeOwned + Serialize>(req: web_sys::Request) -> Result<ResultResponse<T, E>, Error> {
    let resp: web_sys::Response = api_fetch_request(&req).await?;

    let promise = resp.json().map_err(|err| Error::JsValue(err))?;

    let data = JsFuture::from(promise).await.map_err(|err| Error::JsValue(err))?;

    serde_wasm_bindgen::from_value(data).map_err(|err| Error::JsValue(err.into()))
}

pub async fn api_fetch_request(req: &web_sys::Request) -> Result<web_sys::Response, Error> {
    let promise: Promise = web_sys::window().unwrap_throw().fetch_with_request(&req);

    let resp_value = JsFuture::from(promise).await.unwrap_throw();
    let resp: web_sys::Response = resp_value.dyn_into().unwrap_throw();

    let status = resp.status();

    if status != 200 {
        if status == 401 {
            //Force redirect to /unauthorized page
            web_sys::window()
                .unwrap_throw()
                .location()
                .set_href("/user/unauthorized")
                .unwrap_throw();

            //The client won't see this due to redirect
            Err(Error::AuthForbidden)
        } else {
            if let Ok(promise) = resp.json() {
                if let Ok(bad_status) = JsFuture::from(promise).await {
                    if let Ok(status) = serde_wasm_bindgen::from_value::<HttpStatus>(bad_status) {
                        if status.message == "AUTH_COMPLETE_REGISTRATION" {
                            //Force redirect to /complete-registration page
                            web_sys::window()
                                .unwrap_throw()
                                .location()
                                .set_href("/user/complete-registration")
                                .unwrap_throw();
                            
                            //The client won't see this due to redirect
                            return Err(Error::AuthCompleteRegistration)
                        }
                    }
                }
            }
            Err(Error::HttpStatusCodeOnly(status))
        }

    } else {
        Ok(resp)
    }
}

pub fn get_request_with_headers<A: AsRef<str>, B: AsRef<str>>(url: &str, pairs: &[(A, B)], data:Option<impl Serialize>) -> Result<web_sys::Request, JsValue> {
    
    let mut req_init = web_sys::RequestInit::new();
    req_init.method("POST");
    req_init.credentials(web_sys::RequestCredentials::Include);

    let req = match data {
        None => {
            let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;

            req
        },
        Some(data) => {
            let json_str = serde_json::to_string(&data).map_err(|err| JsValue::from_str(&err.to_string()))?;
            //req_init.mode(web_sys::RequestMode::Cors);
            req_init.body(Some(&JsValue::from_str(&json_str)));
            let req = web_sys::Request::new_with_str_and_init(url, &req_init)?;

            req.headers().set("Content-Type", "application/json")?;

            req
        }
    };

    let headers = req.headers();

    for (name, value) in pairs.iter() {
        headers.set(name.as_ref(), value.as_ref())?;
    }
    Ok(req)
}

