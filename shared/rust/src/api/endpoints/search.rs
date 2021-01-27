//! routes for the global animation library

use crate::{
    api::{ApiEndpoint, Method},
    domain::search::CreateSearchKeyResponse,
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
