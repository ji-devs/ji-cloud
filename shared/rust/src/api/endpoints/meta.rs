use crate::{
    api::{ApiEndpoint, Method},
    domain::meta::{GetMetadataPath, MetadataResponse},
    error::EmptyError,
};

/// Get metadata.
pub struct Get;
impl ApiEndpoint for Get {
    type Path = GetMetadataPath;
    type Req = ();
    type Res = MetadataResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
