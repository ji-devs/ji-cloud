//! Contains error types for api operations. Enums are declared with HTTP status codes
//! indicating the type of error with optional descriptions.
//!
//! Casting and handling of:
//!     * `sqlx::Error` -- database errors generated in database interactions, mostly in `db` module
//!     * `actix_web::Error` -- server errors, used by actix for error logging
//!     * `anyhow::Error` -- general intermediate error representation

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use shared::error::{EmptyError, MetadataNotFound};
use shared::error::{ServiceError, ServiceKindError};

use crate::db::meta::MetaWrapperError;

mod oauth;
pub use oauth::{GoogleOAuth, OAuth};

mod storage;
pub use storage::Storage;

mod user;
pub use user::{
    Email, NotFound as UserNotFound, Register, Update as UserUpdate, Username, VerifyEmail,
};

pub mod event_arc;
pub use event_arc::EventArc;

use shared::domain::meta::MetaKind;

/// TODO: Remove once all usages have migrated to the new [`ApiError`] struct.
///
/// Represents an error returned by the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseError<T> {
    /// The status code of the error.
    #[serde(skip)]
    pub code: http::StatusCode,

    /// A message describing the error.
    ///
    /// Note: This message is for human readability and is explicitly *not* stable, do not use this message to figure out what error was returned.
    pub message: String,

    /// Any optional additional information.
    #[serde(flatten)]
    pub extra: T,
}
impl<T> std::fmt::Display for ApiResponseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl<T: std::fmt::Debug> std::error::Error for ApiResponseError<T> {}

impl<T: Serialize> From<ApiResponseError<T>> for actix_web::Error {
    fn from(e: ApiResponseError<T>) -> Self {
        let resp = HttpResponse::build(e.code).json(e);
        actix_web::error::InternalError::from_response("", resp).into()
    }
}

impl<T: Default> ApiResponseError<T> {
    /// Creates a new error based off the provided status code
    #[must_use]
    pub fn new(code: http::StatusCode) -> Self {
        Self {
            message: code
                .canonical_reason()
                .unwrap_or("Unknown Error")
                .to_owned(),
            code,
            extra: T::default(),
        }
    }

    /// Creates a new error based off the provided status code and with the provided message.
    #[must_use]
    pub fn with_message(code: http::StatusCode, message: String) -> Self {
        Self {
            message,
            code,
            extra: T::default(),
        }
    }
}

impl<T> ApiResponseError<T> {}

/// Represents an error returned by the api.
// mostly used in this module
#[allow(clippy::module_name_repetitions)]
pub type BasicError = ApiResponseError<EmptyError>;

#[non_exhaustive]
pub enum Auth {
    InternalServerError(anyhow::Error),
    Forbidden,
    ResourceNotFound(String),
}

impl<T: Into<anyhow::Error>> From<T> for Auth {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Auth {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
            Self::ResourceNotFound(message) => {
                BasicError::with_message(http::StatusCode::NOT_FOUND, message).into()
            }
        }
    }
}

pub fn ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    actix_web::error::InternalError::from_response(
        "",
        resp.json(BasicError::new(http::StatusCode::INTERNAL_SERVER_ERROR)),
    )
    .into()
}

#[non_exhaustive]
pub enum Delete {
    Conflict,
    Forbidden,
    ResourceNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Delete {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<Auth> for Delete {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

impl Into<actix_web::Error> for Delete {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Conflict => BasicError::new(http::StatusCode::CONFLICT).into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::ResourceNotFound => BasicError::new(http::StatusCode::NOT_FOUND).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

pub struct Server(pub anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for Server {
    fn from(e: T) -> Self {
        Self(e.into())
    }
}

impl Into<actix_web::Error> for Server {
    fn into(self) -> actix_web::Error {
        ise(self.0)
    }
}

impl From<Auth> for ServiceError {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e.into()),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

#[derive(Debug)]
pub enum ServiceSession {
    InternalServerError(anyhow::Error),
    DisabledService(ServiceKindError),
    Unauthorized,
}

impl<T: Into<anyhow::Error>> From<T> for ServiceSession {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for ServiceSession {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::DisabledService(s) => BasicError::with_message(
                http::StatusCode::NOT_IMPLEMENTED,
                format!("{s} is disabled"),
            )
            .into(),
            Self::Unauthorized => BasicError::new(http::StatusCode::UNAUTHORIZED).into(),
        }
    }
}

#[derive(Debug)]
pub enum Refresh {
    InternalServerError(anyhow::Error),
    PreconditionFailed,
    ResourceNotFound,
}

impl<T: Into<anyhow::Error>> From<T> for Refresh {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Refresh {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::PreconditionFailed => {
                BasicError::new(http::StatusCode::PRECONDITION_FAILED).into()
            }
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
        }
    }
}

pub enum NotFound {
    ResourceNotFound,
    Forbidden,
    InternalServerError(anyhow::Error),
    BadRequest,
}

impl From<Auth> for NotFound {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for NotFound {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for NotFound {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
            Self::BadRequest => BasicError::new(http::StatusCode::BAD_REQUEST).into(),
        }
    }
}

pub enum CategoryUpdate {
    CategoryNotFound,
    ParentCategoryNotFound,
    Cycle,
    OutOfRange,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for CategoryUpdate {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CategoryUpdate {
    fn into(self) -> actix_web::Error {
        match self {
            Self::CategoryNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Category Not Found".to_owned(),
            )
            .into(),

            Self::ParentCategoryNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Parent Category Not Found".to_owned(),
            )
            .into(),

            Self::Cycle => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Would cause a cycle".to_owned(),
            )
            .into(),

            Self::OutOfRange => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Out of range".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[derive(Debug)]
pub enum Upload {
    ResourceNotFound,
    InvalidMedia,
    FileTooLarge,
    StorageClient(Storage),
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Upload {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Upload {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::InvalidMedia => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid Content".to_owned(),
            )
            .into(),
            Self::FileTooLarge => BasicError::with_message(
                http::StatusCode::PAYLOAD_TOO_LARGE,
                "File Exceeds Upload Limit".to_owned(),
            )
            .into(),
            Self::StorageClient(e) => e.into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<Storage> for Upload {
    fn from(e: Storage) -> Self {
        match e {
            Storage::FileTooLarge => Upload::FileTooLarge,
            Storage::InternalServerError(e) => Upload::InternalServerError(e),
            e => Upload::StorageClient(e),
        }
    }
}

pub enum CreateWithMetadata {
    InternalServerError(anyhow::Error),
    Forbidden,
    MissingMetadata(MetadataNotFound),
    ResourceNotFound,
}

impl From<Auth> for CreateWithMetadata {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for CreateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CreateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiResponseError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),
            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),
            Self::InternalServerError(e) => ise(e),
            Self::ResourceNotFound => BasicError::new(http::StatusCode::BAD_REQUEST).into(),
        }
    }
}

pub enum UpdateWithMetadata {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
    Forbidden,
}

impl From<Auth> for UpdateWithMetadata {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for UpdateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UpdateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiResponseError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<MetaWrapperError> for CreateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound {
                    id,
                    index: None,
                    kind,
                    media_group_kind: None,
                })
            }
            MetaWrapperError::MissingTag {
                index,
                media_group_kind,
            } => Self::MissingMetadata(MetadataNotFound {
                id: None,
                index,
                kind: MetaKind::Tag,
                media_group_kind: Some(media_group_kind),
            }),
        }
    }
}

impl From<MetaWrapperError> for UpdateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound {
                    id,
                    index: None,
                    kind,
                    media_group_kind: None,
                })
            }
            MetaWrapperError::MissingTag {
                index,
                media_group_kind,
            } => Self::MissingMetadata(MetadataNotFound {
                id: None,
                index,
                kind: MetaKind::Tag,
                media_group_kind: Some(media_group_kind),
            }),
        }
    }
}

pub enum Tag {
    TakenIndex,
    ResourceNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Tag {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Tag {
    fn into(self) -> actix_web::Error {
        match self {
            Self::TakenIndex => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "Tag index already taken".to_owned(),
            )
            .into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

pub enum UserRecentImage {
    ResourceNotFound,
    Conflict,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for UserRecentImage {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UserRecentImage {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Image not found in recent images list for user".to_owned(),
            )
            .into(),

            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "An image with the same ID already exists in recent images list for user"
                    .to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

pub enum CloneDraft {
    ResourceNotFound,
    UnprocessableEntity,
    IncompleteModules,
    Conflict,
    Forbidden,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for CloneDraft {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<Auth> for CloneDraft {
    fn from(e: Auth) -> Self {
        match e {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}

impl Into<actix_web::Error> for CloneDraft {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),

            Self::UnprocessableEntity => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "Called method not allowed".to_owned(),
            )
            .into(),

            Self::IncompleteModules => BasicError::with_message(
                http::StatusCode::BAD_REQUEST,
                "No activities exist or activities with missing content".to_owned(),
            )
            .into(),

            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "A draft already exists".to_owned(),
            )
            .into(),

            Self::Forbidden => BasicError::new(http::StatusCode::FORBIDDEN).into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

pub enum MediaProcessing {
    InternalServerError(anyhow::Error),
    EventArc(EventArc),
    ResourceNotFound,
}

impl<T: Into<anyhow::Error>> From<T> for MediaProcessing {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for MediaProcessing {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),

            Self::EventArc(e) => e.into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
        }
    }
}

pub enum ReportError {
    InternalServerError(anyhow::Error),
    ResourceNotFound,
    SendEmailFail,
}

impl<T: Into<anyhow::Error>> From<T> for ReportError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for ReportError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::SendEmailFail => BasicError::with_message(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to report to email".to_owned(),
            )
            .into(),
        }
    }
}

pub enum JigCode {
    InternalServerError(anyhow::Error),
    ResourceNotFound,
    Conflict,
    AllCodesUsed,
    Forbidden,
}

impl<T: Into<anyhow::Error>> From<T> for JigCode {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for JigCode {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),

            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "A code already exists for this jig".to_owned(),
            )
            .into(),

            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource not found".to_owned(),
            )
            .into(),

            Self::AllCodesUsed => BasicError::with_message(
                http::StatusCode::SERVICE_UNAVAILABLE,
                "Maximum number of possible codes taken?".to_owned(),
            )
            .into(),

            Self::Forbidden => BasicError::with_message(
                http::StatusCode::FORBIDDEN,
                "User does not have permissions for this jig".to_owned(),
            )
            .into(),
        }
    }
}

impl From<Auth> for JigCode {
    fn from(err: Auth) -> Self {
        match err {
            Auth::InternalServerError(e) => Self::InternalServerError(e),
            Auth::Forbidden => Self::Forbidden,
            Auth::ResourceNotFound(_) => Self::ResourceNotFound,
        }
    }
}
