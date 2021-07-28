use super::ApiEndpoint;
use crate::domain::image::ImageUploadRequest;
use crate::{
    api::Method,
    domain::image::{
        CreateResponse, ImageBrowseQuery, ImageBrowseResponse, ImageCreateRequest, ImageResponse,
        ImageSearchQuery, ImageSearchResponse, ImageUpdateRequest, ImageUploadResponse,
    },
    error::{EmptyError, MetadataNotFound},
};
use http::StatusCode;

pub mod recent;
pub mod tag;
pub mod user;

/// Get an image by ID.
///
/// # Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image with the requested ID is not found.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ImageResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Search for images.
///
/// # Request
/// The request should be supplied as a URL query string.
/// * `kind` field must match the case as represented in the returned json body (`PascalCase`?).
/// * Vector fields, such as `age_ranges` should be given as a comma separated vector (CSV).
///
/// Ex: `?age_ranges=b873b584-efd0-11eb-b4b7-b791fd521ed5,b8388824-efd0-11eb-b4b7-c335e6a1139f,b778a054-efd0-11eb-b4b7-6f7305d76205&page=0`
///
/// # Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`Unimplemented`](http::StatusCode::UNIMPLEMENTED) when the algolia service is disabled.
pub struct Search;
impl ApiEndpoint for Search {
    type Req = ImageSearchQuery;
    type Res = ImageSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Get;
}

/// Browse images.
///
/// # Request
/// The request should be supplied as a URL query string. `kind` field must match the case as
/// represented in the returned json body (`PascalCase`?).
///
/// Ex: `?kind=Canvas&page=0`
///
/// # Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = ImageBrowseQuery;
    type Res = ImageBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/browse";
    const METHOD: Method = Method::Get;
}

/// Create an image.
///
/// # Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ImageCreateRequest;
    type Res = CreateResponse;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Post;
}

/// Upload an image.
///
/// _NOTE_: can be used to update the raw data associated with the image.
///
/// # Errors:
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid. This may be an API server issue.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`Unimplemented`](http::StatusCode::UNIMPLEMENTED) when the s3/gcs service is disabled.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = ImageUploadRequest;
    type Res = ImageUploadResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}/raw";
    const METHOD: Method = Method::Patch;
}

/// Update an image's metadata.
pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = ImageUpdateRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an image.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Delete;
}
