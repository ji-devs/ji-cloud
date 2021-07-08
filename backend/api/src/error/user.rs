use super::{ise, BasicError};
use paperclip::actix::api_v2_errors;
use shared::error::{ApiError, EmptyError, MetadataNotFound};

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
    code = 409,
    code = 420,
    code = 500,
    code = 501
)]
pub enum Register {
    Username(RegisterUsername),
    Email(RegisterEmail),
    Service(super::Service),
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
            Self::Username(e) => e.into(),
            Self::Email(e) => e.into(),
            Self::Service(e) => e.into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<super::Service> for Register {
    fn from(s: super::Service) -> Self {
        match s {
            super::Service::DisabledService(s) => Self::Service(super::Service::DisabledService(s)),
            super::Service::InternalServerError(e) => Self::InternalServerError(e),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided username already exists",
    code = 420,
    description = "Unprocessable Entity: No username was provided",
    code = 500
)]
pub enum RegisterUsername {
    EmptyUsername,
    TakenUsername,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for RegisterUsername {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for RegisterUsername {
    fn into(self) -> actix_web::Error {
        match self {
            Self::EmptyUsername => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "No username was provided".to_owned(),
            )
            .into(),

            Self::TakenUsername => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "Username already taken".to_owned(),
            )
            .into(),

            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl Into<Register> for RegisterUsername {
    fn into(self) -> Register {
        match self {
            Self::InternalServerError(e) => Register::InternalServerError(e),
            username_error => Register::Username(username_error),
        }
    }
}

#[api_v2_errors(
    code = 400,
    code = 409,
    description = "Conflict: Another user with the provided email already exists",
    code = 420,
    description = "Unprocessable Entity: No email was provided",
    code = 500
)]
pub enum RegisterEmail {
    EmptyEmail,
    TakenEmailGoogle,
    TakenEmailBasic,
    InternalServerError(anyhow::Error),
    // TODO: invalid email
}

impl<T: Into<anyhow::Error>> From<T> for RegisterEmail {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for RegisterEmail {
    fn into(self) -> actix_web::Error {
        match self {
            Self::EmptyEmail => BasicError::with_message(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "No email was provided".to_owned(),
            )
            .into(),
            Self::TakenEmailGoogle | Self::TakenEmailBasic => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "Email already taken".to_owned(),
            )
            .into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl Into<Register> for RegisterEmail {
    fn into(self) -> Register {
        match self {
            Self::InternalServerError(e) => Register::InternalServerError(e),
            email_error => Register::Email(email_error),
        }
    }
}
