/*

   the _status functions return the status without doing side effects
   the non_status versions do side effects based on the status (e.g. redirect to no-auth page)
*/

use std::fmt::Debug;
use std::{
    any::TypeId,
    error::Error,
    fmt::{self, Display},
    future, result,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    fetch_with_headers_and_data, fetch_with_headers_and_data_abortable, Response,
};
use web_sys::File;

pub use awsm_web::loaders::helpers::{spawn_handle, AbortController, FutureHandle};
use shared::error::ApiError;

pub const POST: &str = "POST";
pub const GET: &str = "GET";

pub type IsAborted = bool;

const DESERIALIZE_OK: &str = "couldn't deserialize ok in fetch";

#[derive(Debug, Serialize, Deserialize)]
pub enum FetchError<T: Debug + Display> {
    Connection,
    Parse,
    Response(ApiError<T>),
}
impl<T> fmt::Display for FetchError<T>
where
    T: fmt::Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Connection => write!(f, "Error when communicating with server"),
            FetchError::Parse => write!(f, "Error parsing server response"),
            FetchError::Response(e) => write!(f, "Error: {e}"),
        }
    }
}
impl<T> Error for FetchError<T> where T: Debug + Display {}

pub type ApiResult<T, E> = Result<T, FetchError<E>>;

// extension trait to make calling the API very convenient
#[async_trait(?Send)]
pub trait ApiEndpointExt {
    type Path: PathParts;
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize + 'static;
    type Err: DeserializeOwned + Serialize + Error + 'static;

    // const EXTPATH: &'static str;
    const EXT_METHOD: Method;

    /**** WITH AUTH ****/
    async fn api_with_auth(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> ApiResult<Self::Res, Self::Err> {
        let (resp, status) = Self::api_with_auth_status(path, data).await;

        side_effect_status_code(status).await;

        resp
    }
    async fn api_with_auth_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (ApiResult<Self::Res, Self::Err>, u16) {
        Self::api_with_auth_status_abortable(None, path, data)
            .await
            .unwrap_ji()
    }
    async fn api_with_auth_abortable(
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<ApiResult<Self::Res, Self::Err>, IsAborted> {
        let resp = Self::api_with_auth_status_abortable(abort_controller, path, data).await;

        if let Ok((_, status)) = resp {
            side_effect_status_code(status).await;
        }

        resp.map(|(resp, _)| resp)
    }
    async fn api_with_auth_status_abortable(
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<(ApiResult<Self::Res, Self::Err>, u16), IsAborted> {
        if let Ok(token) = env_var("LOCAL_API_AUTH_OVERRIDE") {
            Self::api_with_token_status_abortable(&token, abort_controller, path, data).await
        } else {
            let csrf = load_csrf_token().unwrap_or_default();

            let (url, data) = api_get_query(&path.get_filled(), Self::EXT_METHOD, data);

            match fetch_with_headers_and_data(
                &url,
                Self::EXT_METHOD.as_str(),
                true,
                &[(CSRF_HEADER_NAME, &csrf)],
                data,
            )
            .await
            {
                Ok(res) => {
                    let status = res.status();

                    if res.ok() {
                        Ok((Self::res_to_json(res).await, status))
                    } else {
                        let res: Result<ApiError<Self::Err>, _> = Self::res_to_json(res).await;
                        // since `!res.ok()` this should be an Err even if parsing succeeded.
                        let res = error_response_to_err(res);
                        Ok((res, status))
                    }
                }
                Err(err) => {
                    if err.is_abort() {
                        Err(true)
                    } else {
                        Ok((Err(FetchError::Connection), 0))
                    }
                }
            }
        }
    }

    /**** NO AUTH ****/
    async fn api_no_auth(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> ApiResult<Self::Res, Self::Err> {
        let (resp, status) = Self::api_no_auth_status(path, data).await;

        side_effect_status_code(status).await;

        resp
    }
    async fn api_no_auth_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (ApiResult<Self::Res, Self::Err>, u16) {
        let (url, data) = api_get_query(&path.get_filled(), Self::EXT_METHOD, data);

        let res = fetch_with_data(&url, Self::EXT_METHOD.as_str(), false, data)
            .await
            .unwrap_ji();

        let status = res.status();

        if res.ok() {
            (Self::res_to_json(res).await, status)
        } else {
            let res: Result<ApiError<Self::Err>, _> = Self::res_to_json(res).await;
            // since `!res.ok()` this should be an Err even if parsing succeeded.
            let res = error_response_to_err(res);
            (res, status)
        }
    }

    /**** WITH BEARER TOKEN ****/
    async fn api_with_token(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> ApiResult<Self::Res, Self::Err> {
        let (resp, status) = Self::api_with_token_status(token, path, data).await;

        side_effect_status_code(status).await;

        resp
    }
    async fn api_with_token_status(
        token: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (ApiResult<Self::Res, Self::Err>, u16) {
        Self::api_with_token_status_abortable(token, None, path, data)
            .await
            .unwrap_ji()
    }
    async fn api_with_token_status_abortable(
        token: &str,
        abort_controller: Option<&AbortController>,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> Result<(ApiResult<Self::Res, Self::Err>, u16), IsAborted> {
        let bearer = format!("Bearer {}", token);

        let (url, data) = api_get_query(&path.get_filled(), Self::EXT_METHOD, data);

        match fetch_with_headers_and_data_abortable(
            &url,
            Self::EXT_METHOD.as_str(),
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
                    Ok((Self::res_to_json(res).await, status))
                } else {
                    let res: Result<ApiError<Self::Err>, _> = Self::res_to_json(res).await;
                    // since `!res.ok()` this should be an Err even if parsing succeeded.
                    let res = error_response_to_err(res);
                    Ok((res, status))
                }
            }
            Err(err) => {
                if err.is_abort() {
                    Err(true)
                } else {
                    Ok((Err(FetchError::Connection), 0))
                }
            }
        }
    }

    /**** WITH CREDENTIALS ****/
    //used in cases where we have the cookie but not the token
    //really just used for login and registration, to get the token via oauth flow
    async fn api_no_auth_with_credentials(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> ApiResult<Self::Res, Self::Err> {
        let (resp, status) = Self::api_no_auth_with_credentials_status(path, data).await;

        side_effect_status_code(status).await;

        resp
    }
    async fn api_no_auth_with_credentials_status(
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (ApiResult<Self::Res, Self::Err>, u16) {
        let (url, data) = api_get_query(&path.get_filled(), Self::EXT_METHOD, data);

        let res = fetch_with_data(&url, Self::EXT_METHOD.as_str(), true, data)
            .await
            .unwrap_ji();

        let status = res.status();

        if res.ok() {
            (
                Ok(Self::res_to_json(res).await.expect_ji(DESERIALIZE_OK)),
                status,
            )
        } else {
            let res: Result<ApiError<Self::Err>, _> = Self::res_to_json(res).await;
            // since `!res.ok()` this should be an Err even if parsing succeeded.
            let res = error_response_to_err(res);
            (res, status)
        }
    }

    /**** WITH BASIC ****/
    //really just used with login - see https://datatracker.ietf.org/doc/html/rfc7617#section-2
    async fn api_with_basic_token(
        user_id: &str,
        password: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> ApiResult<Self::Res, Self::Err> {
        let (resp, status) = Self::api_with_basic_token_status(user_id, password, path, data).await;

        side_effect_status_code(status).await;

        resp
    }
    async fn api_with_basic_token_status(
        user_id: &str,
        password: &str,
        path: Self::Path,
        data: Option<Self::Req>,
    ) -> (ApiResult<Self::Res, Self::Err>, u16) {
        let credentials = format!("{}:{}", user_id, password);
        let token = base64::encode(credentials.as_bytes());
        let basic = format!("Basic {}", token);

        let (url, data) = api_get_query(&path.get_filled(), Self::EXT_METHOD, data);

        let res = fetch_with_headers_and_data(
            &url,
            Self::EXT_METHOD.as_str(),
            true,
            &[("Authorization", &basic)],
            data,
        )
        .await
        .unwrap_ji();

        let status = res.status();

        if res.ok() {
            (
                Ok(Self::res_to_json(res).await.expect_ji(DESERIALIZE_OK)),
                status,
            )
        } else {
            let res: Result<ApiError<Self::Err>, _> = Self::res_to_json(res).await;
            // since `!res.ok()` this should be an Err even if parsing succeeded.
            let res = error_response_to_err(res);
            (res, status)
        }
    }

    // TODO: use specialization once stable instead.
    /// Similar to awsm_web::loaders::fetch::Response::json_from_str, but treats an empty string as valid input for `()`
    async fn res_to_json<T>(res: Response) -> Result<T, FetchError<Self::Err>>
    where
        T: DeserializeOwned + 'static,
    {
        let mut text = res.text().await.map_err(|_| FetchError::Parse)?;
        if TypeId::of::<T>() == TypeId::of::<()>() {
            if text.is_empty() {
                text = String::from("null");
            }
        }
        serde_json::from_str(&text).map_err(|e| {
            log::info!("Parsing error: {e}");
            FetchError::Parse
        })
    }
}

// takes a result and return an error whether the input is Ok or Err. Calls into in both cases.
// useful in after parsing the servers error response, we need an Err whether the parsing was successful or not.
fn error_response_to_err<T, E>(res: Result<ApiError<E>, FetchError<E>>) -> Result<T, FetchError<E>>
where
    E: Debug + Display,
{
    let err = match res {
        Ok(error) => FetchError::Response(error),
        Err(error) => error,
    };

    Err(err)
}

// impl the extension for all endpoints
impl<T: ApiEndpoint> ApiEndpointExt for T {
    type Path = T::Path;
    type Req = T::Req;
    type Res = T::Res;
    type Err = T::Err;

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
) -> result::Result<(), awsm_web::errors::Error> {
    let (resp, status) = upload_file_gcs_status(url, file, abort_controller).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn upload_file_gcs_status(
    url: &str,
    file: &File,
    abort_controller: Option<&AbortController>,
) -> (result::Result<(), awsm_web::errors::Error>, u16) {
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
pub async fn api_upload_file(
    endpoint: &str,
    file: &File,
    method: Method,
) -> result::Result<(), ()> {
    let (resp, status) = api_upload_file_status(endpoint, file, method).await;

    side_effect_status_code(status).await;

    resp
}

pub async fn api_upload_file_status(
    endpoint: &str,
    file: &File,
    method: Method,
) -> (result::Result<(), ()>, u16) {
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
