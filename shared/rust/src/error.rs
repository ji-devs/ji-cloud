//! Home of the error types.

use serde::{Deserialize, Serialize};

use crate::domain::meta::MetaKind;
use crate::media::MediaGroupKind;

/// auth errors
#[deprecated]
pub mod auth {
    #[deprecated]
    pub use super::EmptyError as RegisterError;
}

/// Represents an error returned by the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError<T> {
    /// The status code of the error.
    #[serde(with = "http_serde::status_code")]
    pub code: http::StatusCode,

    /// A message describing the error.
    ///
    /// Note: This message is for human readability and is explicitly *not* stable, do not use this message to figure out what error was returned.
    pub message: String,

    /// Any optional additional information.
    #[serde(flatten)]
    pub extra: T,
}

#[cfg(feature = "backend")]
impl<T: Serialize> From<ApiError<T>> for actix_web::Error {
    fn from(e: ApiError<T>) -> Self {
        let resp = actix_web::HttpResponse::build(e.code).json(e);
        actix_web::error::InternalError::from_response("", resp).into()
    }
}

impl<T: Default> ApiError<T> {
    /// Creates a new error based off the provided status code
    #[must_use]
    pub fn new(code: http::StatusCode) -> Self {
        Self {
            message: code
                .canonical_reason()
                .unwrap_or("Unknown Error")
                .to_owned(),
            code,
            extra: T::default(),
        }
    }

    /// Creates a new error based off the provided status code and with the provided message.
    #[must_use]
    pub fn with_message(code: http::StatusCode, message: String) -> Self {
        Self {
            message,
            code,
            extra: T::default(),
        }
    }
}

impl<T> ApiError<T> {}

/// An `extra` error type that represents "no extension"
#[derive(Serialize, Deserialize, Debug, Default, thiserror::Error)]
#[error("EmptyError")]
pub struct EmptyError {}

/// Metadata associated with this operation could not be found.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataNotFound {
    /// The (Optional) id of the item.
    pub id: Option<uuid::Uuid>,
    /// The (Optional) index of the item.
    pub index: Option<i16>,
    /// The item's kind.
    pub kind: MetaKind,
    /// The (Optional) media group of the item where the error originated, for metadata types that
    /// are split per media group kind.
    pub media_group_kind: Option<MediaGroupKind>,
}
