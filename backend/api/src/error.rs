use actix_web::HttpResponse;
use paperclip::actix::api_v2_errors;
use shared::error::{auth::RegisterErrorKind, ApiError, EmptyError, MetadataNotFound};

use crate::db::meta::MetaWrapperError;

/// Represents an error returned by the api.
pub type BasicError = ApiError<EmptyError>;

pub fn ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    resp.json(BasicError::new(
        actix_http::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
    .into()
}

#[non_exhaustive]
#[api_v2_errors(code = 401, code = 403, code = 404, code = 500)]
pub enum DeleteError {
    Conflict,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for DeleteError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for DeleteError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Conflict => BasicError::new(http::StatusCode::CONFLICT).into(),
            Self::InternalServerError(e) => crate::error::ise(e),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500)]
pub struct ServerError(pub anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for ServerError {
    fn from(e: T) -> Self {
        Self(e.into())
    }
}

impl Into<actix_web::Error> for ServerError {
    fn into(self) -> actix_web::Error {
        crate::error::ise(self.0)
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
            Self::InternalServerError(e) => crate::error::ise(e),
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
pub enum NotFoundError {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for NotFoundError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for NotFoundError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ResourceNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Resource Not Found".to_owned(),
            )
            .into(),
            Self::InternalServerError(e) => crate::error::ise(e),
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
pub enum CategoryUpdateError {
    CategoryNotFound,
    ParentCategoryNotFound,
    Cycle,
    OutOfRange,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for CategoryUpdateError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CategoryUpdateError {
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

            Self::InternalServerError(e) => crate::error::ise(e),
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
pub enum UploadError {
    ResourceNotFound,
    InvalidMedia,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for UploadError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UploadError {
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
            Self::InternalServerError(e) => crate::error::ise(e),
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
pub enum CreateWithMetadataError {
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
}

impl<T: Into<anyhow::Error>> From<T> for CreateWithMetadataError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CreateWithMetadataError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::MissingMetadata(data) => ApiError {
                code: http::StatusCode::UNPROCESSABLE_ENTITY,
                message: "Metadata not Found".to_owned(),
                extra: data,
            }
            .into(),
            Self::InternalServerError(e) => crate::error::ise(e),
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
pub enum UpdateWithMetadataError {
    ResourceNotFound,
    InternalServerError(anyhow::Error),
    MissingMetadata(MetadataNotFound),
}

impl<T: Into<anyhow::Error>> From<T> for UpdateWithMetadataError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for UpdateWithMetadataError {
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
            Self::InternalServerError(e) => crate::error::ise(e),
        }
    }
}

impl From<MetaWrapperError> for CreateWithMetadataError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => Self::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                Self::MissingMetadata(MetadataNotFound { id, kind })
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateWithMetadataError {
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
pub enum RegisterError {
    RegisterError(RegisterErrorKind),
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for RegisterError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for RegisterError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::RegisterError(kind) => {
                let message = match kind {
                    RegisterErrorKind::EmptyDisplayName => "No username was provided",
                    RegisterErrorKind::TakenEmail => "Email already taken",
                    RegisterErrorKind::TakenId => "Firebase ID already associated with a user",
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
            Self::InternalServerError(e) => crate::error::ise(e),
        }
    }
}
