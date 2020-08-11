#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum RegisterError {
    EmptyDisplayName,
    TakenEmail,
    TakenId,
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<RegisterError> for actix_web::Error {
    fn from(e: RegisterError) -> actix_web::Error {
        match e {
            RegisterError::InternalServerError(e) => anyhow_to_ise(e),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum FirebaseError {
    MissingBearerToken,
    InvalidToken,
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<FirebaseError> for actix_web::Error {
    fn from(e: FirebaseError) -> Self {
        match e {
            FirebaseError::InternalServerError(e) => anyhow_to_ise(e),
            e => HttpResponse::Unauthorized().json(e).into(),
        }
    }
}

from_anyhow![RegisterError, FirebaseError];
