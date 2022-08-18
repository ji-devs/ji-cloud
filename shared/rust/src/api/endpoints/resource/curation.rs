//! routes for the resource curation by admin

use crate::{
    api::Method,
    domain::{
        resource::curation::{
            CommentId, ResourceCurationCommentRequest, ResourceCurationCommentResponse,
            ResourceCurationData, ResourceCurationUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;
/// Get a curation data by Resource ID.
pub struct GetCuration;
impl ApiEndpoint for GetCuration {
    type Req = ();
    type Res = ResourceCurationData;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/curation";
    const METHOD: Method = Method::Get;
}

/// Update a curation data by Resource ID.
pub struct UpdateCuration;
impl ApiEndpoint for UpdateCuration {
    type Req = ResourceCurationUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/curation";
    const METHOD: Method = Method::Patch;
}

/// Submit a comment by Resource ID.
pub struct CreateComment;
impl ApiEndpoint for CreateComment {
    type Req = ResourceCurationCommentRequest;
    type Res = CreateResponse<CommentId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/curation/comment";
    const METHOD: Method = Method::Post;
}

/// Get a comment by comment ID.
pub struct GetComment;
impl ApiEndpoint for GetComment {
    type Req = ();
    type Res = ResourceCurationCommentResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/curation/comment/{comment_id}";
    const METHOD: Method = Method::Get;
}
