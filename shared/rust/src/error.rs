//! Home of the error types.

#[cfg(feature = "backend")]
use paperclip::actix::api_v2_errors;

/// Generates a [`From`](std::from::From) impl to convert from [`Into<anyhow::Error>`] to an enum
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

pub mod audio;
pub mod auth;
pub mod category;
pub mod image;
pub mod jig;
/// Stand-in for the `!` (`Never`) type, while waiting for it to be stablized.
#[derive(Serialize, Deserialize)]
pub enum Infallible {}

#[cfg(feature = "backend")]
impl From<Infallible> for actix_web::Error {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

/// User errors.
pub mod user {

    /// The user does not exist.
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct NoSuchUserError {}
}

use serde::{Deserialize, Serialize};

/// Converts from an [`anyhow::Error`] to a http `InternalServerError`.
#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = actix_web::HttpResponse::InternalServerError();
    // put the contents of the error into an extension to avoid the client seeing what the error is, and so that the log picks it up.
    resp.extensions_mut().insert(e);
    resp.into()
}

/// Represents an error from the backend.
#[cfg_attr(feature = "backend", api_v2_errors)]
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

/// Error occurred while getting a single resource.a
#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
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
#[cfg_attr(feature = "backend", api_v2_errors)]
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

// fixme: (if this breaking change is ever possible): Use a `CommonError` type
#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
#[derive(Serialize, Deserialize)]
/// Error occurred while creating a Resource.
pub enum CreateError<T = Infallible> {
    /// User has insufficient permissions to create the Resource.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),

    /// Some more specific error has occured (see the documentation for the specific `T`)
    Extra(T),
}

#[cfg(feature = "backend")]
impl<T: Into<actix_web::Error>> From<CreateError<T>> for actix_web::Error {
    fn from(e: CreateError<T>) -> actix_web::Error {
        match e {
            CreateError::InternalServerError(e) => anyhow_to_ise(e),
            CreateError::Forbidden => actix_web::HttpResponse::Forbidden().into(),
            CreateError::Extra(e) => e.into(),
        }
    }
}

// fixme: (if this breaking change is ever possible): Use a `CommonError` type
#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
#[derive(Serialize, Deserialize)]
/// Error occurred while updating a Resource.
pub enum UpdateError<T = Infallible> {
    /// The Resource was not found.
    NotFound,

    /// User has insufficient permissions to update the Resource.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),

    /// Some more specific error has occured (see the documentation for the specific `T`)
    Extra(T),
}

#[cfg(feature = "backend")]
impl<T: Into<actix_web::Error>> From<UpdateError<T>> for actix_web::Error {
    fn from(e: UpdateError<T>) -> actix_web::Error {
        match e {
            UpdateError::InternalServerError(e) => anyhow_to_ise(e),
            UpdateError::NotFound => actix_web::HttpResponse::NotFound().into(),
            UpdateError::Forbidden => actix_web::HttpResponse::Forbidden().into(),
            UpdateError::Extra(e) => e.into(),
        }
    }
}

#[non_exhaustive]
#[cfg_attr(feature = "backend", api_v2_errors)]
#[derive(Serialize, Deserialize)]
/// A common error type
pub enum CommonError {
    /// A Resource was not found.
    NotFound,

    /// User has insufficient permissions to perform an action on the Resource.
    Forbidden,

    /// An internal server error occurred.
    #[serde(skip)]
    InternalServerError(anyhow::Error),
}

#[cfg(feature = "backend")]
impl From<CommonError> for actix_web::Error {
    fn from(e: CommonError) -> actix_web::Error {
        match e {
            CommonError::InternalServerError(e) => anyhow_to_ise(e),
            CommonError::NotFound => actix_web::HttpResponse::NotFound().into(),
            CommonError::Forbidden => actix_web::HttpResponse::Forbidden().into(),
        }
    }
}

from_anyhow![GetError, DeleteError, CreateError, UpdateError, CommonError];
