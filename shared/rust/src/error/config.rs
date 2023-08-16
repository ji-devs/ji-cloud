use mymacros::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[cfg(feature = "backend")]
use crate::error::TransientError;

#[cfg(feature = "backend")]
use actix_web::{
    body::BoxBody,
    error::{JsonPayloadError, PathError, QueryPayloadError},
    HttpResponse, ResponseError,
};

/// Represents actix-web config errors
#[allow(missing_docs)]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ConfigError {
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Invalid JSON body"))]
    JsonPayloadError {
        #[cfg(feature = "backend")]
        #[serde(skip)]
        #[from]
        source: TransientError<JsonPayloadError>,
        a: bool,
    },
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Invalid query"))]
    QueryPayloadError {
        #[cfg(feature = "backend")]
        #[serde(skip)]
        #[from]
        source: TransientError<QueryPayloadError>,
        a: bool,
    },
    #[cfg_attr(feature = "backend", error(transparent))]
    #[cfg_attr(not(feature = "backend"), error("Invalid path parameters"))]
    PathError {
        #[cfg(feature = "backend")]
        #[serde(skip)]
        #[from]
        source: TransientError<PathError>,
        a: bool,
    },
}

#[cfg(feature = "backend")]
impl From<JsonPayloadError> for ConfigError {
    fn from(error: JsonPayloadError) -> Self {
        Self::from(TransientError::from(error))
    }
}

#[cfg(feature = "backend")]
impl From<QueryPayloadError> for ConfigError {
    fn from(error: QueryPayloadError) -> Self {
        Self::from(TransientError::from(error))
    }
}

#[cfg(feature = "backend")]
impl From<PathError> for ConfigError {
    fn from(error: PathError) -> Self {
        Self::from(TransientError::from(error))
    }
}

#[cfg(feature = "backend")]
impl ResponseError for ConfigError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::JsonPayloadError { source } => source.status_code(),
            Self::QueryPayloadError { source } => source.status_code(),
            Self::PathError { source } => source.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
