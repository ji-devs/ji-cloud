//! Endpoints for ProDevUnit
use crate::{
    api::Method,
    domain::{
        pro_dev::unit::{
            CreateProDevUnitPath, DeleteProDevUnitPath, GetProDevUnitDraftPath,
            GetProDevUnitLivePath, ProDevUnit, ProDevUnitCreateRequest, ProDevUnitId,
            ProDevUnitUpdateRequest, UpdateProDevUnitPath,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Get an Pro Dev Unit on a draft Pro Dev
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Pro Dev Unit or the parent Pro Dev doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = ProDevUnit;
    type Path = GetProDevUnitDraftPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get an Pro Dev Unit on a live Pro Dev
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Pro Dev Unit or the parent Pro Dev doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = ProDevUnit;
    type Path = GetProDevUnitLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Add a Pro Dev Unit to a draft Pro Dev.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct Create;
impl ApiEndpoint for Create {
    type Path = CreateProDevUnitPath;
    type Req = ProDevUnitCreateRequest;
    type Res = CreateResponse<ProDevUnitId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Update an Pro Dev Units to a draft Pro Dev.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct Update;
impl ApiEndpoint for Update {
    type Path = UpdateProDevUnitPath;
    type Req = ProDevUnitUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete an Pro Dev Unit from a draft Pro Dev.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Pro Dev Unit or parent Pro Dev does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = DeleteProDevUnitPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
