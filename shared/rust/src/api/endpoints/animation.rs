//! routes for the global animation library

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        animation::{AnimationCreateRequest, AnimationId, AnimationResponse},
        CreateResponse,
    },
    error::EmptyError,
};

/// Get an animation by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = AnimationResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/animation/{id}";
    const METHOD: Method = Method::Get;
}
/// Create an animation.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = AnimationCreateRequest;
    type Res = CreateResponse<AnimationId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/animation";
    const METHOD: Method = Method::Post;
}

/// Upload an animation
/// Note: can be used to update the raw data associated with the animation.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/animation/{id}/raw";
    const METHOD: Method = Method::Put;
}

/// Delete an animation.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/animation/{id}";
    const METHOD: Method = Method::Delete;
}
