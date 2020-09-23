use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::image::{
        CreateRequest, CreateResponse, GetResponse, SearchQuery, SearchResponse, UpdateRequest,
        UpdateResponse,
    },
    error::image::{CreateError, DeleteError, GetError, SearchError, UpdateError},
};

/// Get an image by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = GetResponse;
    type Err = GetError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Search for images.
pub struct Search;
impl ApiEndpoint for Search {
    type Req = SearchQuery;
    type Res = SearchResponse;
    type Err = SearchError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Get;
}

/// Create an image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateRequest;
    type Res = CreateResponse;
    type Err = CreateError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Post;
}

/// Update an image's metadata.
pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = UpdateRequest;
    type Res = UpdateResponse;
    type Err = UpdateError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an image.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = DeleteError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Delete;
}
