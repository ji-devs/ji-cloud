use crate::domain::billing::{SchoolNameId, SchoolNameValue};
use crate::error::billing::BillingError;
use crate::error::TransientError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "backend")]
use actix_web::{body::BoxBody, HttpResponse, ResponseError};

#[allow(missing_docs)]
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AccountError {
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Internal server error"))]
    InternalServerError(
        #[serde(skip)]
        #[from]
        TransientError<anyhow::Error>,
    ),
    #[error("User already has an existing account")]
    UserHasAccount,
    #[error("A school name of {0} already exists")]
    SchoolNameExists(SchoolNameValue),
    #[error("A school using a name with ID {0} already exists")]
    SchoolExists(SchoolNameId),
    #[error("{0}")]
    NotFound(String),
    #[error("Forbidden")]
    Forbidden,
}

#[cfg(feature = "backend")]
impl ResponseError for AccountError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::InternalServerError { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => http::StatusCode::NOT_FOUND,
            Self::Forbidden => http::StatusCode::FORBIDDEN,
            _ => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<anyhow::Error> for AccountError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<AccountError> for BillingError {
    fn from(value: AccountError) -> Self {
        match value {
            AccountError::Forbidden => Self::Forbidden,
            AccountError::InternalServerError(error) => Self::InternalServerError(error),
            error => TransientError::from(anyhow::Error::from(error)).into(),
        }
    }
}
