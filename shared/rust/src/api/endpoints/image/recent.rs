//! Routes for a user's recently used images list
//! Note: this assumes that the image referred to exists or is valid.

use crate::{
    api::{endpoints::ApiEndpoint, Method},
    domain::image::recent::{
        UserRecentImageDeletePath, UserRecentImageListPath, UserRecentImageListRequest,
        UserRecentImageListResponse, UserRecentImageResponse, UserRecentImageUpsertPath,
        UserRecentImageUpsertRequest,
    },
    error::EmptyError,
};

/// List recent images for the user.
/// Note: `limit` query is optional.
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
pub struct List;

impl ApiEndpoint for List {
    type Path = UserRecentImageListPath;
    type Req = UserRecentImageListRequest;
    type Res = UserRecentImageListResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update or add an entry in the list of recent user images.
/// Invoking this bumps the entry to the top of the recent images list.
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image doesn't exist in the user's recent images list.
///
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
pub struct Put;

// TODO: Move ID into request body
// TODO: grab req/res from above
impl ApiEndpoint for Put {
    type Path = UserRecentImageUpsertPath;
    type Req = UserRecentImageUpsertRequest;
    type Res = UserRecentImageResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Remove an entry from the list of recent user images.
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed.
pub struct Delete;

impl ApiEndpoint for Delete {
    type Path = UserRecentImageDeletePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
