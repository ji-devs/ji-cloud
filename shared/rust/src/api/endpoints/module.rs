use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{
        module::{
            ModuleCreatePath, ModuleCreateRequest, ModuleDeletePath, ModuleDeleteRequest,
            ModuleGetDraftPath, ModuleGetLivePath, ModuleId, ModuleResponse, ModuleUpdateRequest,
            ModuleUploadPath,
        },
        CreateResponse,
    },
    error::EmptyError,
};

/// Get a Module by it's concrete ID.
///
/// # Authorization
/// No authorization required.
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Path = ModuleGetLivePath;
    type Req = ();
    type Res = ModuleResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a Module by it's concrete ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfAsset` for owned JIGs or Playlists
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Path = ModuleGetDraftPath;
    type Req = ();
    type Res = ModuleResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope).
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct Create;
impl ApiEndpoint for Create {
    type Path = ModuleCreatePath;
    type Req = ModuleCreateRequest;
    type Res = CreateResponse<ModuleId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Update a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope).
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](crate::Uuid) or the request is missing/invalid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the jig or module does not exist.
pub struct Update;
impl ApiEndpoint for Update {
    type Path = ModuleUploadPath;
    type Req = ModuleUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope).
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the jig or module does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](crate::Uuid).
pub struct Delete;
impl ApiEndpoint for Delete {
    type Path = ModuleDeletePath;
    type Req = ModuleDeleteRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
