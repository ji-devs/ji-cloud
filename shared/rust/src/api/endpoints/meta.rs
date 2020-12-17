use crate::{
    api::{ApiEndpoint, Method},
    domain::meta::MetadataResponse,
};

/// Get metadata.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = MetadataResponse;
    type Err = ();
    const PATH: &'static str = "/v1/metadata";
    const METHOD: Method = Method::Get;
}
