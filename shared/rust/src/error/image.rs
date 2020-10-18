//! Errors for image routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
use crate::domain::meta::MetaKind;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Error occurred while getting a single image.
#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum GetError {
    /// The image does not exist.
    NotFound,

    /// The user has insufficient permissions to access the image.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
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
/// Error occurred while searching for images.
pub enum SearchError {
    // todo: is this variant useful?
    /// The user has insufficient permissions to search for images.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<SearchError> for actix_web::Error {
    fn from(e: SearchError) -> actix_web::Error {
        match e {
            SearchError::InternalServerError(e) => anyhow_to_ise(e),
            SearchError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while creating an image.
pub enum CreateError {
    /// A given item of metadata doesn't exist.
    NonExistantMetadata {
        /// The (Optional) id of the item.
        id: Option<Uuid>,
        /// The item's kind.
        kind: MetaKind,
    },

    /// User has insufficient permissions to create an image.
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
            e @ CreateError::NonExistantMetadata { .. } => {
                HttpResponse::UnprocessableEntity().json(e).into()
            }
            CreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while uploading an image.
pub enum UploadError {
    /// User has insufficient permissions to upload the image.
    Forbidden,

    /// The image does not exist.
    NotFound,

    /// Couldn't parse the body into an image
    InvalidImage,

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
            UploadError::InvalidImage => HttpResponse::UnprocessableEntity().json(e).into(),
            UploadError::Forbidden => HttpResponse::Forbidden().json(e).into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while updating an image.
pub enum UpdateError {
    /// A given item of metadata doesn't exist.
    NonExistantMetadata {
        /// The (Optional) id of the item.
        id: Option<Uuid>,
        /// The item's kind.
        kind: MetaKind,
    },

    /// The category was not found.
    NotFound,

    /// User has insufficient permissions to update the image.
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
            e @ UpdateError::NonExistantMetadata { .. } => {
                HttpResponse::UnprocessableEntity().json(e).into()
            }
            UpdateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while deleting an image.
pub enum DeleteError {
    /// User has insufficient permissions to delete the image.
    Forbidden,

    // todo: [conflict] With what?
    /// Deleting the image would cause a conflict.
    Conflict,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
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

from_anyhow![
    GetError,
    SearchError,
    CreateError,
    UpdateError,
    DeleteError,
    UploadError
];
