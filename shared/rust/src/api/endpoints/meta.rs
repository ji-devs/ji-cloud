use crate::{
    api::{ApiEndpoint, Method},
    domain::meta::MetadataResponse,
    error::EmptyError,
};

/// Get metadata.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = MetadataResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/metadata";
    const METHOD: Method = Method::Get;
}
