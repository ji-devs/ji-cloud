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

pub mod recent;
pub mod tag;
pub mod user;

/// Get an image by ID.
pub struct Get;

impl ApiEndpoint for Get {
    type Req = ();
    type Res = ImageResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/{id}";
    const METHOD: Method = Method::Get;
}

/// Search for images.
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
/// The request should be supplied as a URL query string.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = ImageBrowseQuery;
    type Res = ImageBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/browse";
    const METHOD: Method = Method::Get;
}

/// Create an image.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ImageCreateRequest;
    type Res = CreateResponse;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/image";
    const METHOD: Method = Method::Post;
}

/// Upload an image.
/// _NOTE_: can be used to update the raw data associated with the image.
///
/// Errors:
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
