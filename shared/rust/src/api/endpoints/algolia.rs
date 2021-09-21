use super::ApiEndpoint;

use crate::{
    api::Method,
    domain::{
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

/// Create the service for the goolge cloud scheduler
///
/// # Authorization
///
///
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/update_algolia";
    const METHOD: Method = Method::Get;
}