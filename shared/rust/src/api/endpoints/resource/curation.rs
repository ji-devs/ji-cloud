//! routes for the resource curation by admin

use crate::{
    api::Method,
    domain::{
        resource::curation::{
            CommentId, ResourceCurationCommentCreatePath, ResourceCurationCommentGetPath,
            ResourceCurationCommentRequest, ResourceCurationCommentResponse, ResourceCurationData,
            ResourceCurationPath, ResourceCurationUpdatePath, ResourceCurationUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;
/// Get a curation data by Resource ID.
pub struct GetCuration;
impl ApiEndpoint for GetCuration {
    type Path = ResourceCurationPath;
    type Req = ();
    type Res = ResourceCurationData;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a curation data by Resource ID.
pub struct UpdateCuration;
impl ApiEndpoint for UpdateCuration {
    type Path = ResourceCurationUpdatePath;
    type Req = ResourceCurationUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Submit a comment by Resource ID.
pub struct CreateComment;
impl ApiEndpoint for CreateComment {
    type Path = ResourceCurationCommentCreatePath;
    type Req = ResourceCurationCommentRequest;
    type Res = CreateResponse<CommentId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get a comment by comment ID.
pub struct GetComment;
impl ApiEndpoint for GetComment {
    type Path = ResourceCurationCommentGetPath;
    type Req = ();
    type Res = ResourceCurationCommentResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
