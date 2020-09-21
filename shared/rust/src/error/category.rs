//! Errors for category routes.

#[cfg(feature = "backend")]
use super::anyhow_to_ise;
#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while geting a category.
pub enum CategoryGetError {
    /// The user has insufficient permissions to access the route or category.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<CategoryGetError> for actix_web::Error {
    fn from(e: CategoryGetError) -> actix_web::Error {
        match e {
            CategoryGetError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryGetError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while creating a category.
pub enum CategoryCreateError {
    /// The given parent category does not exist.
    ParentCategoryNotFound,

    /// The user has insufficient permissions to create categories.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<CategoryCreateError> for actix_web::Error {
    fn from(e: CategoryCreateError) -> actix_web::Error {
        match e {
            CategoryCreateError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryCreateError::ParentCategoryNotFound => HttpResponse::NotFound().into(),
            CategoryCreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while deleting a category.
pub enum CategoryDeleteError {
    /// The user has insufficient permissions to delete the category.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<CategoryDeleteError> for actix_web::Error {
    fn from(e: CategoryDeleteError) -> actix_web::Error {
        match e {
            CategoryDeleteError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryDeleteError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while updating a category.
pub enum CategoryUpdateError {
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
impl From<CategoryUpdateError> for actix_web::Error {
    fn from(e: CategoryUpdateError) -> actix_web::Error {
        match e {
            CategoryUpdateError::InternalServerError(e) => anyhow_to_ise(e),
            e @ CategoryUpdateError::CategoryNotFound
            | e @ CategoryUpdateError::ParentCategoryNotFound => {
                HttpResponse::NotFound().json(e).into()
            }
            CategoryUpdateError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

from_anyhow![
    CategoryGetError,
    CategoryCreateError,
    CategoryDeleteError,
    CategoryUpdateError
];
