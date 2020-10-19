//! Errors for JIG routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while creating a JIG.
pub enum CreateError {
    /// User has insufficient permissions to create the JIG.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<CreateError> for actix_web::Error {
    fn from(e: CreateError) -> actix_web::Error {
        match e {
            CreateError::InternalServerError(e) => anyhow_to_ise(e),
            CreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while updating an image.
pub enum UpdateError {
    /// The JIG was not found.
    NotFound,

    /// User has insufficient permissions to update the JIG.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<UpdateError> for actix_web::Error {
    fn from(e: UpdateError) -> actix_web::Error {
        match e {
            UpdateError::InternalServerError(e) => anyhow_to_ise(e),
            UpdateError::NotFound => HttpResponse::NotFound().into(),
            UpdateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

from_anyhow![CreateError, UpdateError];
