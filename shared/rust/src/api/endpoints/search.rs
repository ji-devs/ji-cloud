//! routes for the global animation library

use crate::{
    api::{ApiEndpoint, Method},
    domain::search::{CreateSearchKeyResponse, WebImageSearchQuery, WebImageSearchResponse},
    error::EmptyError,
};

/// Create a search key.
pub struct CreateKey;
impl ApiEndpoint for CreateKey {
    type Req = ();
    type Res = CreateSearchKeyResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/search/key";
    const METHOD: Method = Method::Post;
}

/// Search for images over the web.
pub struct WebImageSearch;
impl ApiEndpoint for WebImageSearch {
    type Req = WebImageSearchQuery;
    type Res = WebImageSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/search/web/image";
    const METHOD: Method = Method::Get;
}
