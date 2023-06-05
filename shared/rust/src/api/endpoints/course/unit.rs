//! Endpoints for CourseUnit
use crate::{
    api::Method,
    domain::{
        course::unit::{
            CourseUnit, CourseUnitCreateRequest, CourseUnitId, CourseUnitUpdateRequest,
            CreateCourseUnitPath, DeleteCourseUnitPath, GetCourseUnitDraftPath,
            GetCourseUnitLivePath, UpdateCourseUnitPath,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Get an Course Unit on a draft Course
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Course Unit or the parent Course doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = CourseUnit;
    type Path = GetCourseUnitDraftPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get an Course Unit on a live Course
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Course Unit or the parent Course doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = CourseUnit;
    type Path = GetCourseUnitLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Add a Course Unit to a draft Course.
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
    type Path = CreateCourseUnitPath;
    type Req = CourseUnitCreateRequest;
    type Res = CreateResponse<CourseUnitId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Update an Course Units to a draft Course.
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
    type Path = UpdateCourseUnitPath;
    type Req = CourseUnitUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete an Course Unit from a draft Course.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the Course Unit or parent Course does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = DeleteCourseUnitPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
