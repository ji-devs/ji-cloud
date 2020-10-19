//! Home of the error types.

/// Generates a `From` impl to convert from `Into<anyhow::Error>` to an enum
/// with a `InternalServerError(anyhow::Error)` variant.
macro_rules! from_anyhow {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl<T: Into<anyhow::Error>> From<T> for $t {
                fn from(e: T) -> Self {
                    Self::InternalServerError(e.into())
                }
            }
        )+
    };
}

pub mod auth;
pub mod category;
pub mod image;
pub mod jig;

/// User errors.
pub mod user {

    /// The user does not exist.
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct NoSuchUserError {}
}

use serde::{Deserialize, Serialize};

/// Converts from an [`anyhow::Error`] to a http `InternalServerError`.
///
/// [`anyhow::Error`]: ../../anyhow/struct.Error.html
#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = actix_web::HttpResponse::InternalServerError();
    // put the contents of the error into an extension to avoid the client seeing what the error is, and so that the log picks it up.
    resp.extensions_mut().insert(e);
    resp.into()
}

/// Represents an error from the backend.
pub struct InternalServerError(pub anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for InternalServerError {
    fn from(e: T) -> Self {
        InternalServerError(e.into())
    }
}

#[cfg(feature = "backend")]
impl From<InternalServerError> for actix_web::Error {
    fn from(e: InternalServerError) -> actix_web::Error {
        anyhow_to_ise(e.0)
    }
}

/// Error occurred while getting a single resource.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum GetError {
    /// The resource does not exist.
    NotFound,

    /// The user has insufficient permissions to access the resource.
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
            GetError::NotFound => actix_web::HttpResponse::NotFound().into(),
            GetError::Forbidden => actix_web::HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
/// Error occurred while deleting a resource.
pub enum DeleteError {
    /// User has insufficient permissions to delete the resource.
    Forbidden,

    /// Deleting the resource would cause a conflict.
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
            DeleteError::Forbidden => actix_web::HttpResponse::Forbidden().into(),
            DeleteError::Conflict => actix_web::HttpResponse::Conflict().into(),
        }
    }
}

from_anyhow![GetError, DeleteError];
