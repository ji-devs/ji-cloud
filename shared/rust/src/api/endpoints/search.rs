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
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if missing/invalid auth was provided.
/// [`Unimplemented`](http::StatusCode::UNIMPLEMENTED) If the route is not configured.
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
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if missing/invalid auth was provided.
/// [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request was not provided in a proper format
pub struct WebImageSearch;
impl ApiEndpoint for WebImageSearch {
    type Req = WebImageSearchQuery;
    type Res = WebImageSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/search/web/image";
    const METHOD: Method = Method::Get;
}
