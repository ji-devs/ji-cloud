//! Errors for audio routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while uploading an audio file.
pub enum UploadError {
    /// User has insufficient permissions to upload the audio file.
    Forbidden,

    /// The audio file does not exist.
    NotFound,

    /// Couldn't parse the body into an mp3
    InvalidAudio,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<UploadError> for actix_web::Error {
    fn from(e: UploadError) -> actix_web::Error {
        match e {
            UploadError::InternalServerError(e) => anyhow_to_ise(e),
            UploadError::NotFound => HttpResponse::NotFound().json(e).into(),
            // should this be 400 instead?
            UploadError::InvalidAudio => HttpResponse::UnprocessableEntity().json(e).into(),
            UploadError::Forbidden => HttpResponse::Forbidden().json(e).into(),
        }
    }
}

from_anyhow![UploadError];
