//! Home of the error types.

mod account;
mod billing;
mod config;
mod service;

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[cfg(feature = "backend")]
use actix_web::{body::BoxBody, HttpResponse, ResponseError};

use mymacros::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::meta::MetaKind;
use crate::media::MediaGroupKind;

pub use account::AccountError;
pub use billing::BillingError;
pub use config::ConfigError;
pub use service::{ServiceError, ServiceKindError};

/// An `extra` error type that represents "no extension"
#[derive(Serialize, Deserialize, Debug, Default, thiserror::Error)]
#[error("EmptyError")]
pub struct EmptyError {}

/// Metadata associated with this operation could not be found.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataNotFound {
    /// The (Optional) id of the item.
    pub id: Option<crate::Uuid>,
    /// The (Optional) index of the item.
    pub index: Option<i16>,
    /// The item's kind.
    pub kind: MetaKind,
    /// The (Optional) media group of the item where the error originated, for metadata types that
    /// are split per media group kind.
    pub media_group_kind: Option<MediaGroupKind>,
}
impl Display for MetadataNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Metadata not found")
    }
}
impl Error for MetadataNotFound {}

/// Helper trait
pub trait IntoAnyhow<T> {
    /// Convert `self` into a result with an anyhow error
    ///
    /// # Errors
    ///
    /// Maps the error in a Result into an [`anyhow::Error`].
    fn into_anyhow(self) -> anyhow::Result<T>;
}

// Blanket impl
impl<T, E> IntoAnyhow<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static + Into<anyhow::Error>,
{
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(Into::into)
    }
}

/// Useful for serializing errors that don't implement Serialize, or errors where we don't want the
/// error details to be transported to the client.
#[derive(Debug, Error, mymacros::Serialize, mymacros::Deserialize)]
pub enum TransientError<T>
where
    T: Debug + Display
{
    /// The actual error
    #[error("API error {0}")]
    Error(T),
    /// An error placeholder
    #[error("")]
    Missing,
}

impl<T: Debug + Display> Default for TransientError<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<T: Debug + Display> From<T> for TransientError<T> {
    fn from(value: T) -> Self {
        Self::Error(value)
    }
}

#[cfg(feature = "backend")]
impl<T> TransientError<T>
where
    T: ResponseError + Debug + Display,
{
    #[allow(missing_docs)]
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::Error(inner) => inner.status_code(),
            Self::Missing => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiError<T> {
    ApiError(T),
    ConfigError(ConfigError),
}

#[cfg(feature = "backend")]
impl<T> ResponseError for ApiError<T>
where
    T: ResponseError + Serialize,
{
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::ConfigError(error) => error.status_code(),
            Self::ApiError(error) => error.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl<T: Display> Display for ApiError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigError(error) => write!(f, "{error}"),
            Self::ApiError(error) => write!(f, "{error}"),
        }
    }
}
