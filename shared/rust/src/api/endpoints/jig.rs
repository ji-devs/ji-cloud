use crate::{api::Method, domain::jig::{CreateRequest, CreateResponse, GetResponse, UpdateRequest}, error::{
        jig::{CreateError, UpdateError},
        DeleteError, GetError,
    }};

use super::ApiEndpoint;

/// Get a JIG by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = GetResponse;
    type Err = GetError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Get;
}

/// Create a JIG.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateRequest;
    type Res = CreateResponse;
    type Err = CreateError;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Post;
}

/// Update a JIG.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UpdateRequest;
    type Res = ();
    type Err = UpdateError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete a JIG.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = DeleteError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Delete;
}
