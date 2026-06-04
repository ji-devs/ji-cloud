//! routes for the global animation library

use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{
        animation::{
            AnimationCreatePath, AnimationCreateRequest, AnimationDeletePath, AnimationGetPath,
            AnimationId, AnimationResponse, AnimationUploadPath,
        },
        CreateResponse,
    },
    error::EmptyError,
};

/// Get an animation by ID.
pub struct Get;
impl ApiEndpoint for Get {
    type Path = AnimationGetPath;
    type Req = ();
    type Res = AnimationResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
/// Create an animation.
pub struct Create;
impl ApiEndpoint for Create {
    type Path = AnimationCreatePath;
    type Req = AnimationCreateRequest;
    type Res = CreateResponse<AnimationId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Upload raw animation bytes.
pub struct Upload;
impl ApiEndpoint for Upload {
    type Path = AnimationUploadPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Delete an animation.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Path = AnimationDeletePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
