//! Home of the error types.

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

pub mod auth;

use serde::{Deserialize, Serialize};

use crate::domain::meta::MetaKind;

/// Converts from an [`anyhow::Error`] to a http `InternalServerError`.
#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = actix_web::HttpResponse::InternalServerError();
    // put the contents of the error into an extension to avoid the client seeing what the error is, and so that the log picks it up.
    resp.extensions_mut().insert(e);
    resp.into()
}

/// Represents an error returned by the api.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError<T> {
    /// The status code of the error.
    #[serde(with = "http_serde::status_code")]
    pub code: http::StatusCode,

    /// A message describing the error.
    pub message: String,

    /// Any optional additional information.
    #[serde(flatten)]
    pub extra: T,
}

#[cfg(feature = "backend")]
impl<T: Serialize> From<ApiError<T>> for actix_web::Error {
    fn from(e: ApiError<T>) -> Self {
        actix_web::HttpResponse::build(e.code).json(e).into()
    }
}

impl<T: Default> ApiError<T> {
    /// Creates a new error based off the provided status code
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
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EmptyError {}

/// Metadata associated with this operation could not be found.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MetadataNotFound {
    /// The (Optional) id of the item.
    pub id: Option<uuid::Uuid>,
    /// The item's kind.
    pub kind: MetaKind,
}
