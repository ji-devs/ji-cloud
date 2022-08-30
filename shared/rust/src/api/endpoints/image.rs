use super::ApiEndpoint;
use crate::domain::image::{
    ImageBrowsePath, ImageCreatePath, ImageDeletePath, ImageGetPath, ImagePutPath, ImageSearchPath,
    ImageUpdatePath, ImageUploadPath, ImageUploadRequest,
};
use crate::error::{EmptyError, MetadataNotFound};
use crate::{
    api::Method,
    domain::image::{
        CreateResponse, ImageBrowseQuery, ImageBrowseResponse, ImageCreateRequest, ImageResponse,
        ImageSearchQuery, ImageSearchResponse, ImageUpdateRequest, ImageUploadResponse,
    },
};

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
    type Path = ImageGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for images.
///
/// The request should be supplied as a URL query string. This is handled by `serde` if using a `to_string` call.
/// See [`ImageSearchQuery`](crate::domain::image::ImageSearchQuery) for more more usage details.
///
/// Ex: `?age_ranges=b873b584-efd0-11eb-b4b7-b791fd521ed5,b8388824-efd0-11eb-b4b7-c335e6a1139f,b778a054-efd0-11eb-b4b7-6f7305d76205&page=0`
///
/// # Authorization
/// Standard
///
/// # Errors:
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) when the algolia service is disabled.
pub struct Search;
impl ApiEndpoint for Search {
    type Req = ImageSearchQuery;
    type Res = ImageSearchResponse;
    type Path = ImageSearchPath;
    type Err = EmptyError;
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
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = ImageBrowseQuery;
    type Res = ImageBrowseResponse;
    type Path = ImageBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create an image.
///
/// # Errors:
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ImageCreateRequest;
    type Res = CreateResponse;
    type Path = ImageCreatePath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Post;
}

/// Upload an image.
///
/// # Flow:
///
/// 1. User requests an upload session URI directly to Google Cloud Storage
///     a. User uploads to processing bucket
/// 2. Firestore is notified of `processing = true, ready = false` status at document `uploads/media/global/{id}`
/// 3. Image is processed and uploaded to the final bucket
/// 4. Firestore is notified of `processing = true, ready = true` status at document `uploads/media/global/{id}`
///
/// # Notes:
///
/// * Can be used to update the raw data associated with the image.
/// * If the client wants to re-upload an image after it has been successfully processed, it must repeat
/// the entire flow instead of uploading to the same session URI.
///
/// # Errors:
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is invalid.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid. This may be an API server issue, see #1209.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`501 - NotImplemented`](http::StatusCode::NOT_IMPLEMENTED) when the s3/gcs service is disabled.
pub struct Upload;
impl ApiEndpoint for Upload {
    // raw bytes
    type Req = ImageUploadRequest;
    type Res = ImageUploadResponse;
    type Path = ImageUploadPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Update an image's metadata.
pub struct UpdateMetadata;
impl ApiEndpoint for UpdateMetadata {
    type Req = ImageUpdateRequest;
    type Res = ();
    type Path = ImageUpdatePath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Patch;
}

/// Delete an image.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = ImageDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Update usage of an Image for metric purposes
pub struct PutImageUsage;
impl ApiEndpoint for PutImageUsage {
    type Req = ();
    type Res = ();
    type Path = ImagePutPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}
