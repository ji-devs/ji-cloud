//! Routes for the user image library

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        image::{
            user::{
                UserImageListResponse, UserImageResponse, UserImageUploadRequest,
                UserImageUploadResponse,
            },
            ImageId,
        },
        CreateResponse,
    },
    error::EmptyError,
};

/// List user library images.
pub struct List;
impl ApiEndpoint for List {
    type Req = ();
    type Res = UserImageListResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image";
    const METHOD: Method = Method::Get;
}

/// Get an user library image by ID.
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
    type Req = ();
    type Res = CreateResponse<ImageId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image";
    const METHOD: Method = Method::Post;
}

/// Upload an image to the user image library.
/// Note: can be used to update the raw data associated with the image.
///
/// Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// * [`Unimplemented`](http::StatusCode::UNIMPLEMENTED) when the s3/gcs service is disabled.
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
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/image/{id}";
    const METHOD: Method = Method::Delete;
}
