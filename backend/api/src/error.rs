use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use paperclip::actix::api_v2_errors;
use shared::error::{auth::RegisterErrorKind, ApiError, EmptyError, MetadataNotFound};

use crate::db::meta::MetaWrapperError;

mod oauth;
pub use oauth::{GoogleOAuth, OAuth};

/// Represents an error returned by the api.
// mostly used in this module
#[allow(clippy::clippy::module_name_repetitions)]
pub type BasicError = ApiError<EmptyError>;

pub fn ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    resp.json(BasicError::new(http::StatusCode::INTERNAL_SERVER_ERROR))
        .into()
}

#[non_exhaustive]
#[api_v2_errors(code = 401, code = 403, code = 404, code = 500)]
pub enum Delete {
    Conflict,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Delete {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Delete {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Conflict => BasicError::new(http::StatusCode::CONFLICT).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500)]
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

#[derive(Debug, Copy, Clone)]
pub enum ServiceKind {
    Algolia,
    S3,
    GoogleOAuth,
}

impl Into<actix_web::Error> for ServiceKind {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Algolia => BasicError::with_message(
                http::StatusCode::NOT_IMPLEMENTED,
                "Algolia service is disabled".to_owned(),
            )
            .into(),
            Self::S3 => BasicError::with_message(
                http::StatusCode::NOT_IMPLEMENTED,
                "S3 service is disabled".to_owned(),
            )
            .into(),
            Self::GoogleOAuth => BasicError::with_message(
                http::StatusCode::NOT_IMPLEMENTED,
                "Google OAuth service is disabled".to_owned(),
            )
            .into(),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum Service {
    InternalServerError(anyhow::Error),
    DisabledService(ServiceKind),
}

impl<T: Into<anyhow::Error>> From<T> for Service {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Service {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => ise(e),
            Self::DisabledService(s) => s.into(),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 412,
    code = 500,
    code = 501
)]
#[derive(Debug)]
pub enum Refresh {
    InternalServerError(anyhow::Error),
    DisabledService(ServiceKind),
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
            Self::DisabledService(s) => s.into(),
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

#[api_v2_errors(
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: User not Found",
    code = 500
)]
pub enum UserNotFound {
    UserNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for UserNotFound {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UserNotFound {
    fn into(self) -> actix_web::Error {
        match self {
            Self::UserNotFound => {
                BasicError::with_message(http::StatusCode::NOT_FOUND, "User Not Found".to_owned())
                    .into()
            }
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 500
)]
pub enum NotFound {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
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
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Parent Category Not Found OR category not found",
    code = 420,
    description = "Unprocessable Entity: Cycle OR OutOfRange"
    code = 500
)]
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

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 420, description = "Unprocessable Entity: Invalid Content"
    code = 500
)]
#[derive(Debug)]
pub enum Upload {
    ResourceNotFound,
    InvalidMedia,
    InternalServerError(anyhow::Error),
}

impl Upload {
    pub fn blocking_error(err: BlockingError<Self>) -> Self {
        match err {
            BlockingError::Canceled => anyhow::anyhow!("Thread pool is gone").into(),
            BlockingError::Error(e) => e,
        }
    }
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
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 420,
    description = "Unprocessable Entity: Metadata not Found"
    code = 500
)]
pub enum CreateWithMetadata {
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
}

impl<T: Into<anyhow::Error>> From<T> for CreateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CreateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Resource Not Found",
    code = 420,
    description = "Unprocessable Entity: Metadata not Found"
    code = 500
)]
pub enum UpdateWithMetadata {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
}

impl<T: Into<anyhow::Error>> From<T> for UpdateWithMetadata {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UpdateWithMetadata {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiError {
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
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<MetaWrapperError> for CreateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound { id, kind })
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateWithMetadata {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound { id, kind })
            }
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 420,
    description = "Unprocessable Entity: No username was provided OR "
    "Another user with the provided email already exists OR "
    "Another user with the provided firebase-id already exists OR "
    "Another user with the provided username already exists",
    code = 500
)]
pub enum Register {
    RegisterError(RegisterErrorKind),
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Register {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Register {
    fn into(self) -> actix_web::Error {
        match self {
            Self::RegisterError(kind) => {
                let message = match kind {
                    RegisterErrorKind::EmptyDisplayName => "No username was provided",
                    RegisterErrorKind::TakenEmail => "Email already taken",
                    RegisterErrorKind::TakenUsername => "Username already taken",
                    _ => "Unprocessable Entity",
                };

                ApiError {
                    code: http::StatusCode::UNPROCESSABLE_ENTITY,
                    message: message.to_owned(),
                    extra: shared::error::auth::RegisterError { kind },
                }
            }
            .into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}
