//! Errors for category routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use paperclip::actix::api_v2_errors;

// fixme: if breaking changes can ever be made, replace with [`crate::error::CreateError`]
#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
#[derive(Serialize, Deserialize)]
/// Error occurred while creating a category.
pub enum CreateError {
    /// The given parent category does not exist.
    ParentCategoryNotFound,

    /// The user has insufficient permissions to create categories.
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
            CreateError::ParentCategoryNotFound => HttpResponse::NotFound().into(),
            CreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

// fixme: if breaking changes can ever be made, replace with [`crate::error::UpdateError`]
#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
#[derive(Serialize, Deserialize)]
/// Error occurred while updating a category.
pub enum UpdateError {
    /// The category didn't exist.
    CategoryNotFound,

    /// The *new* parent category didn't exist.
    ParentCategoryNotFound,

    /// The user has insufficient permissions to update the category.
    Forbidden,

    /// The update would create a cycle between this category and its parents.
    Cycle,

    /// Moving the category to the given index would cause a gap between its siblings and itself.
    OutOfRange {
        /// The highest index a category could be moved to in its new parent.
        max: u16,
    },

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<UpdateError> for actix_web::Error {
    fn from(e: UpdateError) -> actix_web::Error {
        match e {
            UpdateError::InternalServerError(e) => anyhow_to_ise(e),
            e @ UpdateError::CategoryNotFound | e @ UpdateError::ParentCategoryNotFound => {
                HttpResponse::NotFound().json(e).into()
            }
            UpdateError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

from_anyhow![CreateError, UpdateError];
