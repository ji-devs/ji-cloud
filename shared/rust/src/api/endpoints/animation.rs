//! routes for the global animation library

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        animation::{
            AnimationCreateRequest, AnimationId, AnimationResponse, AnimationUploadRequest,
            AnimationUploadResponse,
        },
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
///
/// # Flow:
///
/// 1. User requests an upload session URI directly to Google Cloud Storage
///     a. User uploads to processing bucket
/// 2. Firestore is notified of `processing = true, ready = false` status at document `uploads/media/global/{id}`
/// 3. Animation is processed and uploaded to the final bucket
/// 4. Firestore is notified of `processing = true, ready = true` status at document `uploads/media/global/{id}`
///
/// # Notes:
///
/// * Can be used to update the raw data associated with the animation.
/// * If the client wants to re-upload an image after it has been successfully processed, it must repeat
/// the entire flow instead of uploading to the same session URI.
///
/// Errors:
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) when the s3/gcs service is disabled.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = AnimationUploadRequest;
    type Res = AnimationUploadResponse;
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
