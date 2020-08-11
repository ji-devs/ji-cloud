#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum GetError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    NotFound,
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<GetError> for actix_web::Error {
    fn from(e: GetError) -> actix_web::Error {
        match e {
            GetError::InternalServerError(e) => anyhow_to_ise(e),
            GetError::NotFound => HttpResponse::NotFound().into(),
            GetError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CreateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    Forbidden,
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
pub enum UpdateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    NotFound,
    Forbidden,
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

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum DeleteError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    NotFound,
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<DeleteError> for actix_web::Error {
    fn from(e: DeleteError) -> actix_web::Error {
        match e {
            DeleteError::InternalServerError(e) => anyhow_to_ise(e),
            DeleteError::NotFound => HttpResponse::NotFound().into(),
            DeleteError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

from_anyhow![GetError, CreateError, UpdateError, DeleteError,];
