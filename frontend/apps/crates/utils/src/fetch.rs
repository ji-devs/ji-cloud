/*

   the _status functions return the status without doing side effects
   the non_status versions do side effects based on the status (e.g. redirect to no-auth page)
*/

use std::future;

use serde::{de::DeserializeOwned, Serialize};
use shared::api::{method::Method, PathParts};

use crate::{
    env::env_var,
    routes::{Route, UserRoute},
    storage::load_csrf_token,
    unwrap::UnwrapJiExt,
};
use shared::{api::ApiEndpoint, domain::auth::CSRF_HEADER_NAME};

use super::init::settings::SETTINGS;
use async_trait::async_trait;
use awsm_web::loaders::fetch::{
    fetch_upload_file_abortable, fetch_upload_file_with_headers, fetch_with_data,
    fetch_with_headers_and_data, fetch_with_headers_and_data_abortable,
};
use web_sys::File;

pub use awsm_web::loaders::helpers::{spawn_handle, AbortController, FutureHandle};

pub const POST: &str = "POST";
pub const GET: &str = "GET";

pub type IsAborted = bool;

const DESERIALIZE_ERR: &str = "couldn't deserialize error in fetch";
const DESERIALIZE_OK: &str = "couldn't deserialize ok in fetch";

// extension trait to make calling the API very convenient
#[async_trait(?Send)]
pub trait ApiEndpointExt {
    type Path: PathParts;
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;

    // const EXTPATH: &'static str;
    const EXT_METHOD: Method;

    /**** WITH AUTH ****/
    async fn api_with_auth(path: Self::Path, data: Option<Self::Req>) -> anyhow::Result<Self::Res> {
        api_with_auth(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_with_auth_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<Self::Res>, u16) {
        api_with_auth_status(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_with_auth_abortable(
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<anyhow::Result<Self::Res>, IsAborted> {
        api_with_auth_abortable(&path.get_filled(), Self::EXT_METHOD, abort_controller, data).await
    }
    async fn api_with_auth_status_abortable(
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<(anyhow::Result<Self::Res>, u16), IsAborted> {
        api_with_auth_status_abortable(&path.get_filled(), Self::EXT_METHOD, abort_controller, data)
            .await
    }
    //TODO - get rid of this, use specialization
    async fn api_with_auth_empty(path: Self::Path, data: Option<Self::Req>) -> anyhow::Result<()> {
        api_with_auth_empty(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_with_auth_empty_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<()>, u16) {
        api_with_auth_empty_status(&path.get_filled(), Self::EXT_METHOD, data).await
    }

    /**** NO AUTH ****/
    async fn api_no_auth(path: Self::Path, data: Option<Self::Req>) -> anyhow::Result<Self::Res> {
        api_no_auth(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_no_auth_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<Self::Res>, u16) {
        api_no_auth_status(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    //TODO - get rid of this, use specialization
    async fn api_no_auth_empty(path: Self::Path, data: Option<Self::Req>) -> anyhow::Result<()> {
        api_no_auth_empty(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_no_auth_empty_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<()>, u16) {
        api_no_auth_empty_status(&path.get_filled(), Self::EXT_METHOD, data).await
    }

    /**** WITH BEARER TOKEN ****/
    async fn api_with_token(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> anyhow::Result<Self::Res> {
        api_with_token(&path.get_filled(), token, Self::EXT_METHOD, data).await
    }
    async fn api_with_token_status(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<Self::Res>, u16) {
        api_with_token_status(&path.get_filled(), token, Self::EXT_METHOD, data).await
    }
    async fn api_with_token_status_abortable(
        token: &str,
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<(anyhow::Result<Self::Res>, u16), IsAborted> {
        api_with_token_status_abortable(
            &path.get_filled(),
            token,
            Self::EXT_METHOD,
            abort_controller,
            data,
        )
        .await
    }
    //TODO - get rid of this, use specialization
    async fn api_with_token_empty(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> anyhow::Result<()> {
        api_with_token_empty(&path.get_filled(), token, Self::EXT_METHOD, data).await
    }
    async fn api_with_token_empty_status(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<()>, u16) {
        api_with_token_empty_status(&path.get_filled(), token, Self::EXT_METHOD, data).await
    }

    /**** WITH CREDENTIALS ****/
    //used in cases where we have the cookie but not the token
    //really just used for login and registration, to get the token via oauth flow
    async fn api_no_auth_with_credentials(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> anyhow::Result<Self::Res> {
        api_no_auth_with_credentials(&path.get_filled(), Self::EXT_METHOD, data).await
    }
    async fn api_no_auth_with_credentials_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<Self::Res>, u16) {
        api_no_auth_with_credentials_status(&path.get_filled(), Self::EXT_METHOD, data).await
    }

    /**** WITH BASIC ****/
    //really just used with login - see https://datatracker.ietf.org/doc/html/rfc7617#section-2
    async fn api_with_basic_token(
        user_id: &str,
        password: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> anyhow::Result<Self::Res> {
        api_with_basic_token(
            &path.get_filled(),
            user_id,
            password,
            Self::EXT_METHOD,
            data,
        )
        .await
    }
    async fn api_with_basic_token_status(
        user_id: &str,
        password: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (anyhow::Result<Self::Res>, u16) {
        api_with_basic_token_status(
            &path.get_filled(),
            user_id,
            password,
            Self::EXT_METHOD,
            data,
        )
        .await
    }
}

// impl the extension for all endpoints
impl<T: ApiEndpoint> ApiEndpointExt for T {
    type Path = T::Path;
    type Req = T::Req;
    type Res = T::Res;

    const EXT_METHOD: Method = T::METHOD;
}

/////////////////////////////////////////////////////////
// Pure fetch functions
// most of them are meant for calling with the API
// but not all (e.g. file uploading to GCS)
/////////////////////////////////////////////////////////

/**** FILE UPLOADING ****/
//https://cloud.google.com/storage/docs/performing-resumable-uploads#resume-upload
//TODO - resumeable uploads
pub async fn upload_file_gcs(
    url: &str,
    file: &File,
    abort_controller: Option<&AbortController>,
) -> Result<(), awsm_web::errors::Error> {
    let (resp, status) = upload_file_gcs_status(url, file, abort_controller).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn upload_file_gcs_status(
    url: &str,
    file: &File,
    abort_controller: Option<&AbortController>,
) -> (Result<(), awsm_web::errors::Error>, u16) {
    match fetch_upload_file_abortable(url, file, Method::Put.as_str(), abort_controller).await {
        Ok(res) => {
            let status = res.status();

            if res.ok() {
                (Ok(()), status)
            } else {
                (Err(awsm_web::errors::Error::Empty), status)
            }
        }
        Err(err) => (Err(err), 0),
    }
}

//TODO - deprecate! All uploads should go through GCS signed urls
pub async fn api_upload_file(endpoint: &str, file: &File, method: Method) -> Result<(), ()> {
    let (resp, status) = api_upload_file_status(endpoint, file, method).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_upload_file_status(
    endpoint: &str,
    file: &File,
    method: Method,
) -> (Result<(), ()>, u16) {
    let (url, _) = api_get_query::<()>(endpoint, method, None);

    let csrf = load_csrf_token().unwrap_or_default();
    let res = fetch_upload_file_with_headers(
        &url,
        file,
        method.as_str(),
        true,
        &[(CSRF_HEADER_NAME, &csrf)],
    )
    .await
    .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(()), status)
    }
}

/**** WITH AUTH ****/
pub async fn api_with_auth<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (resp, status) = api_with_auth_status(endpoint, method, data).await;

    side_effect_status_code(status).await;

    resp
}
pub async fn api_with_auth_abortable<T, Payload>(
    endpoint: &str,
    method: Method,
    abort_controller: Option<&AbortController>,
    data: Option<Payload>,
) -> Result<anyhow::Result<T>, IsAborted>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let resp = api_with_auth_status_abortable(endpoint, method, abort_controller, data).await;

    if let Ok((_, status)) = resp {
        side_effect_status_code(status).await;
    }

    resp.map(|(resp, _)| resp)
}

pub async fn api_with_auth_status<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> (anyhow::Result<T>, u16)
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    api_with_auth_status_abortable(endpoint, method, None, data)
        .await
        .unwrap_ji()
}

pub async fn api_with_auth_status_abortable<T, Payload>(
    endpoint: &str,
    method: Method,
    abort_controller: Option<&AbortController>,
    data: Option<Payload>,
) -> Result<(anyhow::Result<T>, u16), IsAborted>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    if let Ok(token) = env_var("LOCAL_API_AUTH_OVERRIDE") {
        api_with_token_status_abortable(endpoint, &token, method, abort_controller, data).await
    } else {
        let csrf = load_csrf_token().unwrap_or_default();

        let (url, data) = api_get_query(endpoint, method, data);

        match fetch_with_headers_and_data(
            &url,
            method.as_str(),
            true,
            &[(CSRF_HEADER_NAME, &csrf)],
            data,
        )
        .await
        {
            Ok(res) => {
                let status = res.status();

                if res.ok() {
                    Ok((
                        Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)),
                        status,
                    ))
                } else {
                    Ok((
                        // Err(anyhow::anyhow!(DESERIALIZE_ERR)),
                        Err(anyhow::anyhow!(DESERIALIZE_ERR)),
                        status,
                    ))
                }
            }
            Err(err) => {
                if err.is_abort() {
                    Err(true)
                } else {
                    panic!("request failed but was not aborted");
                }
            }
        }
    }
}
//TODO - get rid of this, use specialization
pub async fn api_with_auth_empty<Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<()>
where
    Payload: Serialize,
{
    let (resp, status) = api_with_auth_empty_status(endpoint, method, data).await;

    side_effect_status_code(status).await;

    resp
}
pub async fn api_with_auth_empty_status<Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> (Result<(), anyhow::Error>, u16)
where
    Payload: Serialize,
{
    if let Ok(token) = env_var("LOCAL_API_AUTH_OVERRIDE") {
        api_with_token_empty_status(endpoint, &token, method, data).await
    } else {
        let csrf = load_csrf_token().unwrap_or_default();

        let (url, data) = api_get_query(endpoint, method, data);

        let res = fetch_with_headers_and_data(
            &url,
            method.as_str(),
            true,
            &[(CSRF_HEADER_NAME, &csrf)],
            data,
        )
        .await
        .unwrap_ji();

        let status = res.status();

        if res.ok() {
            (Ok(()), status)
        } else {
            (Err(anyhow::anyhow!(DESERIALIZE_ERR)), status)
        }
    }
}

/**** NO AUTH ****/
pub async fn api_no_auth<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (resp, status) = api_no_auth_status(endpoint, method, data).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_no_auth_status<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> (anyhow::Result<T>, u16)
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (
            Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)),
            status,
        )
    } else {
        (
            // Err(anyhow::anyhow!(res.json_from_str().await.expect_ji(DESERIALIZE_ERR))),
            Err(anyhow::anyhow!(DESERIALIZE_ERR)),
            status,
        )
    }
}
//TODO - get rid of this, use specialization
pub async fn api_no_auth_empty<Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> Result<(), anyhow::Error>
where
    Payload: Serialize,
{
    let (resp, status) = api_no_auth_empty_status(endpoint, method, data).await;

    side_effect_status_code(status).await;

    resp
}

//TODO - get rid of this, use specialization
pub async fn api_no_auth_empty_status<Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> (Result<(), anyhow::Error>, u16)
where
    Payload: Serialize,
{
    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), false, data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(anyhow::anyhow!(DESERIALIZE_ERR)), status)
    }
}
/**** WITH BEARER TOKEN ****/
pub async fn api_with_token<T, Payload>(
    endpoint: &str,
    token: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (resp, status) = api_with_token_status(endpoint, token, method, data).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_with_token_status<T, Payload>(
    endpoint: &str,
    token: &str,
    method: Method,
    data: Option<Payload>,
) -> (anyhow::Result<T>, u16)
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    api_with_token_status_abortable(endpoint, token, method, None, data)
        .await
        .unwrap_ji()
}

pub async fn api_with_token_status_abortable<T, Payload>(
    endpoint: &str,
    token: &str,
    method: Method,
    abort_controller: Option<&AbortController>,
    data: Option<Payload>,
) -> Result<(anyhow::Result<T>, u16), IsAborted>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let bearer = format!("Bearer {}", token);

    let (url, data) = api_get_query(endpoint, method, data);

    match fetch_with_headers_and_data_abortable(
        &url,
        method.as_str(),
        true,
        abort_controller,
        &[("Authorization", &bearer)],
        data,
    )
    .await
    {
        Ok(res) => {
            let status = res.status();

            if res.ok() {
                Ok((
                    Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)),
                    status,
                ))
            } else {
                Ok((Err(anyhow::anyhow!(DESERIALIZE_ERR)), status))
            }
        }
        Err(err) => {
            if err.is_abort() {
                Err(true)
            } else {
                panic!("request failed but was not aborted");
            }
        }
    }
}
//TODO - get rid of this, use specialization
pub async fn api_with_token_empty<Payload>(
    endpoint: &str,
    token: &str,
    method: Method,
    data: Option<Payload>,
) -> Result<(), anyhow::Error>
where
    Payload: Serialize,
{
    let (resp, status) = api_with_token_empty_status(endpoint, token, method, data).await;

    side_effect_status_code(status).await;

    resp
}
pub async fn api_with_token_empty_status<Payload>(
    endpoint: &str,
    token: &str,
    method: Method,
    data: Option<Payload>,
) -> (Result<(), anyhow::Error>, u16)
where
    Payload: Serialize,
{
    let bearer = format!("Bearer {}", token);

    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_headers_and_data(
        &url,
        method.as_str(),
        true,
        &[("Authorization", &bearer)],
        data,
    )
    .await
    .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (Ok(()), status)
    } else {
        (Err(anyhow::anyhow!(DESERIALIZE_ERR)), status)
    }
}
/**** WITH CREDENTIALS ****/
//used in cases where we have the cookie but not the token
//really just used for login and registration, to get the token via oauth flow
pub async fn api_no_auth_with_credentials<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (resp, status) = api_no_auth_with_credentials_status(endpoint, method, data).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_no_auth_with_credentials_status<T, Payload>(
    endpoint: &str,
    method: Method,
    data: Option<Payload>,
) -> (anyhow::Result<T>, u16)
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_data(&url, method.as_str(), true, data)
        .await
        .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (
            Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)),
            status,
        )
    } else {
        (Err(anyhow::anyhow!(DESERIALIZE_ERR)), status)
    }
}
/**** WITH BASIC ****/
//really just used with login - see https://datatracker.ietf.org/doc/html/rfc7617#section-2
pub async fn api_with_basic_token<T, Payload>(
    endpoint: &str,
    user_id: &str,
    password: &str,
    method: Method,
    data: Option<Payload>,
) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let (resp, status) =
        api_with_basic_token_status(endpoint, user_id, password, method, data).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_with_basic_token_status<T, Payload>(
    endpoint: &str,
    user_id: &str,
    password: &str,
    method: Method,
    data: Option<Payload>,
) -> (anyhow::Result<T>, u16)
where
    T: DeserializeOwned + Serialize,
    Payload: Serialize,
{
    let credentials = format!("{}:{}", user_id, password);
    let token = base64::encode(credentials.as_bytes());
    let basic = format!("Basic {}", token);

    let (url, data) = api_get_query(endpoint, method, data);

    let res = fetch_with_headers_and_data(
        &url,
        method.as_str(),
        true,
        &[("Authorization", &basic)],
        data,
    )
    .await
    .unwrap_ji();

    let status = res.status();

    if res.ok() {
        (
            Ok(res.json_from_str().await.expect_ji(DESERIALIZE_OK)),
            status,
        )
    } else {
        (Err(anyhow::anyhow!(DESERIALIZE_ERR)), status)
    }
}

/////////////////////////////////////////////////////////
// Helper functions
/////////////////////////////////////////////////////////

fn api_get_query<T: Serialize>(
    endpoint: &str,
    method: Method,
    data: Option<T>,
) -> (String, Option<T>) {
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

//made pub just in case, but rarely ever called from the outside
//helpful for debugging sometimes too
pub async fn side_effect_status_code(status_code: u16) {
    match status_code {
        403 | 401 => {
            Route::User(UserRoute::NoAuth).redirect();
            // don't return so that this error is not handled, redirection is enough
            future::pending::<()>().await;
        }
        _ => {}
    }
}
