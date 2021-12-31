//! routes for the jig curation by admin

use crate::{
    api::Method,
    domain::{
        jig::curation::{
            CommentId, JigCurationCommentRequest, JigCurationCommentResponse, JigCurationData,
            JigCurationUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;
/// Get a curation data by JIG ID.
pub struct GetCuration;
impl ApiEndpoint for GetCuration {
    type Req = ();
    type Res = JigCurationData;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/curation";
    const METHOD: Method = Method::Get;
}

/// Update a curation data by JIG ID.
pub struct UpdateCuration;
impl ApiEndpoint for UpdateCuration {
    type Req = JigCurationUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/curation";
    const METHOD: Method = Method::Patch;
}

/// Submit a comment by JIG ID.
pub struct CreateComment;
impl ApiEndpoint for CreateComment {
    type Req = JigCurationCommentRequest;
    type Res = CreateResponse<CommentId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/curation/comment";
    const METHOD: Method = Method::Post;
}

/// Get a comment by comment ID.
pub struct GetComment;
impl ApiEndpoint for GetComment {
    type Req = ();
    type Res = JigCurationCommentResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/curation/comment/{comment_id}";
    const METHOD: Method = Method::Get;
}
