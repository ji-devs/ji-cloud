use crate::error::TransientError;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;

#[cfg(feature = "backend")]
use actix_web::{body::BoxBody, HttpResponse, ResponseError};

#[allow(missing_docs)]
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ServiceError {
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Internal server error"))]
    InternalServerError(
        #[serde(skip)]
        #[from]
        TransientError<anyhow::Error>,
    ),
    #[error("{0}")]
    DisabledService(ServiceKindError),
    #[error("Forbidden")]
    Forbidden,
    #[error("Resource not found")]
    ResourceNotFound,
}

#[cfg(feature = "backend")]
impl ResponseError for ServiceError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::InternalServerError { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::DisabledService(_) => http::StatusCode::NOT_IMPLEMENTED,
            Self::Forbidden => http::StatusCode::FORBIDDEN,
            Self::ResourceNotFound => http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.into())
    }
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Display, Serialize, Deserialize)]
pub enum ServiceKindError {
    #[strum(serialize = "Algolia")]
    Algolia,
    #[strum(serialize = "S3")]
    S3,
    #[strum(serialize = "Google Cloud Storage")]
    GoogleCloudStorage,
    #[strum(serialize = "Google Cloud EventArc")]
    GoogleCloudEventArc,
    #[strum(serialize = "Google OAuth")]
    GoogleOAuth,
    #[strum(serialize = "Google Cloud Access Key Store")]
    GoogleCloudAccessKeyStore,
    #[strum(serialize = "Sendgrid Mail")]
    Mail,
    #[strum(serialize = "Firebase Cloud Messaging")]
    FirebaseCloudMessaging,
    #[strum(serialize = "Media Upload Cleaner")]
    UploadCleaner,
    #[strum(serialize = "Google Translate")]
    GoogleTranslate,
    #[strum(serialize = "Stripe")]
    Stripe,
}

#[cfg(feature = "backend")]
impl ResponseError for ServiceKindError {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::NOT_IMPLEMENTED
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
