#[cfg(feature = "backend")]
use super::anyhow_to_ise;
use crate::domain::image::meta::MetaKind;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum GetOneError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    NotFound,
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<GetOneError> for actix_web::Error {
    fn from(e: GetOneError) -> actix_web::Error {
        match e {
            GetOneError::InternalServerError(e) => anyhow_to_ise(e),
            GetOneError::NotFound => HttpResponse::NotFound().into(),
            GetOneError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum GetError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<GetError> for actix_web::Error {
    fn from(e: GetError) -> actix_web::Error {
        match e {
            GetError::InternalServerError(e) => anyhow_to_ise(e),
            GetError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CreateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    MissingMetadata {
        id: Option<Uuid>,
        kind: MetaKind,
    },
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<CreateError> for actix_web::Error {
    fn from(e: CreateError) -> actix_web::Error {
        match e {
            CreateError::InternalServerError(e) => anyhow_to_ise(e),
            e @ CreateError::MissingMetadata { .. } => {
                HttpResponse::UnprocessableEntity().json(e).into()
            }
            CreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum UpdateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    MissingMetadata {
        id: Option<Uuid>,
        kind: MetaKind,
    },
    MissingCategory(Option<Uuid>),
    NotFound,
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<UpdateError> for actix_web::Error {
    fn from(e: UpdateError) -> actix_web::Error {
        match e {
            UpdateError::InternalServerError(e) => anyhow_to_ise(e),
            UpdateError::NotFound => HttpResponse::NotFound().into(),
            e @ UpdateError::MissingMetadata { .. } | e @ UpdateError::MissingCategory(_) => {
                HttpResponse::UnprocessableEntity().json(e).into()
            }
            UpdateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum DeleteError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    Forbidden,
    Conflict,
}

#[cfg(feature = "backend")]
impl From<DeleteError> for actix_web::Error {
    fn from(e: DeleteError) -> actix_web::Error {
        match e {
            DeleteError::InternalServerError(e) => anyhow_to_ise(e),
            DeleteError::Forbidden => HttpResponse::Forbidden().into(),
            DeleteError::Conflict => HttpResponse::Conflict().into(),
        }
    }
}

from_anyhow![GetOneError, GetError, CreateError, UpdateError, DeleteError,];
