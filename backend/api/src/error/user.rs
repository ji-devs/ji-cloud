use super::{ise, BasicError, Service, ServiceSession};
use paperclip::actix::api_v2_errors;

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
    Email(Email),
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

impl From<Service> for Register {
    fn from(s: Service) -> Self {
        match s {
            Service::DisabledService(s) => Self::Service(Service::DisabledService(s)),
            Service::InternalServerError(e) => Self::InternalServerError(e),
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
#[derive(Debug)]
pub enum Email {
    EmptyEmail,
    TakenEmailGoogle,
    // two types but
    TakenEmailBasic,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for Email {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Email {
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

impl Into<Register> for Email {
    fn into(self) -> Register {
        match self {
            Self::InternalServerError(e) => Register::InternalServerError(e),
            email_error => Register::Email(email_error),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum VerifyEmail {
    ServiceSession(ServiceSession),
    Email(Email),
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for VerifyEmail {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for VerifyEmail {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ServiceSession(e) => e.into(),
            Self::Email(e) => e.into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}

impl From<ServiceSession> for VerifyEmail {
    fn from(e: ServiceSession) -> Self {
        Self::ServiceSession(e)
    }
}
