//! routes for the global animation library

use crate::{
    api::{ApiEndpoint, Method},
    domain::search::{CreateSearchKeyResponse, WebImageSearchQuery, WebImageSearchResponse},
    error::EmptyError,
};

/// Create a search key.
///
/// # Authorization
///
/// standard
///
/// # Errors
///
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if missing/invalid auth was provided.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) if the route is not configured.
pub struct CreateKey;
impl ApiEndpoint for CreateKey {
    type Req = ();
    type Res = CreateSearchKeyResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/search/key";
    const METHOD: Method = Method::Post;
}

/// Search for images over the web.
///
/// # Authorization
///
/// standard
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request was not provided in a proper format
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if missing/invalid auth was provided.
pub struct WebImageSearch;
impl ApiEndpoint for WebImageSearch {
    type Req = WebImageSearchQuery;
    type Res = WebImageSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/search/web/image";
    const METHOD: Method = Method::Get;
}
