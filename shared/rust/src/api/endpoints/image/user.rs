//! Routes for the user image library

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        image::{
            user::{
                UserImageCreatePath, UserImageCreateRequest, UserImageDeletePath, UserImageGetPath,
                UserImageListPath, UserImageListQuery, UserImageListResponse, UserImageResponse,
                UserImageUploadPath,
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
    type Path = UserImageListPath;
    type Req = UserImageListQuery;
    type Res = UserImageListResponse;
    type Err = EmptyError;
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
    type Path = UserImageGetPath;
    type Req = ();
    type Res = UserImageResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create an user library image.
pub struct Create;
impl ApiEndpoint for Create {
    type Path = UserImageCreatePath;
    type Req = UserImageCreateRequest;
    type Res = CreateResponse<ImageId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Upload raw image bytes to the user image library.
pub struct Upload;
impl ApiEndpoint for Upload {
    type Path = UserImageUploadPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
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
    type Path = UserImageDeletePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
