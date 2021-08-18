//! Routes for the user image library

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        image::{
            user::{
                UserImageCreateRequest, UserImageListQuery, UserImageListResponse,
                UserImageResponse, UserImageUploadRequest, UserImageUploadResponse,
            },
            ImageId,
        },
        CreateResponse,
    },
    error::EmptyError,
};

/// List user library images.
///
/// # Notes
/// * Request includes an optional query, called as a query string.
pub struct List;
impl ApiEndpoint for List {
    type Req = UserImageListQuery;
    type Res = UserImageListResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image";
    const METHOD: Method = Method::Get;
}

/// Get an user library image by ID.
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image with the requested ID is not found for the user.
/// Note that it will still return NOT_FOUND if an user image with the ID exists but is not owner by the
/// requesting user.
/// * TODO other errors here...
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserImageResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Create an user library image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = UserImageCreateRequest;
    type Res = CreateResponse<ImageId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image";
    const METHOD: Method = Method::Post;
}

/// Upload an image to the user image library.
/// # Flow:
///
/// 1. User requests an upload session URI directly to Google Cloud Storage
///     a. User uploads to processing bucket
/// 2. Firestore is notified of `processing = true, ready = false` status at document `uploads/media/user/{id}`
/// 3. Animation is processed and uploaded to the final bucket
/// 4. Firestore is notified of `processing = true, ready = true` status at document `uploads/media/user/{id}`
///
/// # Notes:
///
/// * Can be used to update the raw data associated with the image.
/// * If the client wants to re-upload an image after it has been successfully processed, it must repeat
/// the entire flow instead of uploading to the same session URI.
///
/// # Errors:
///
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) when the s3/gcs service is disabled.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = UserImageUploadRequest;
    type Res = UserImageUploadResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image/{id}/raw";
    const METHOD: Method = Method::Put;
}

/// Delete an image from the user library.
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image with the requested ID is not found for the user.
/// Note that it will still return NOT_FOUND if an user image with the ID exists but is not owner by the
/// requesting user.
/// * TODO other errors here...
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image/{id}";
    const METHOD: Method = Method::Delete;
}
