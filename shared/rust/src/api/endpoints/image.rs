use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::image::{CreateRequest, CreateResponse, GetOneResponse, GetResponse, UpdateRequest},
    error::image::{CreateError, DeleteError, GetOneError, UpdateError, GetError},
};

pub mod meta;

pub struct GetOne;
impl ApiEndpoint for GetOne {
    type Req = ();
    type Res = GetOneResponse;
    type Err = GetOneError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = GetResponse;
    type Err = GetError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateRequest;
    type Res = CreateResponse;
    type Err = CreateError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Post;
}

pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = UpdateRequest;
    type Res = ();
    type Err = UpdateError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Patch;
}

pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = DeleteError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Delete;
}
