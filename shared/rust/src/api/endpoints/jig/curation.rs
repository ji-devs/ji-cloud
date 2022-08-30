//! routes for the jig curation by admin

use crate::{
    api::Method,
    domain::{
        jig::curation::{
            CommentId, JigCurationCommentCreatePath, JigCurationCommentGetPath,
            JigCurationCommentRequest, JigCurationCommentResponse, JigCurationData,
            JigCurationPath, JigCurationUpdatePath, JigCurationUpdateRequest,
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
    type Path = JigCurationPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a curation data by JIG ID.
pub struct UpdateCuration;
impl ApiEndpoint for UpdateCuration {
    type Req = JigCurationUpdateRequest;
    type Res = ();
    type Path = JigCurationUpdatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Submit a comment by JIG ID.
pub struct CreateComment;
impl ApiEndpoint for CreateComment {
    type Req = JigCurationCommentRequest;
    type Res = CreateResponse<CommentId>;
    type Path = JigCurationCommentCreatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get a comment by comment ID.
pub struct GetComment;
impl ApiEndpoint for GetComment {
    type Req = ();
    type Res = JigCurationCommentResponse;
    type Path = JigCurationCommentGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
