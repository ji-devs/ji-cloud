use crate::api::Method;
pub use macros::{make_path_parts, PathPart};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;
use uuid::Uuid;

//  add something for auth required?

/// Represents a A endpoint that the backend will support, and how to call it.
pub trait ApiEndpoint {
    /// The path type for this endpoint.
    type Path: PathParts;

    /// The request type for this endpoint.
    type Req: Serialize;

    /// The response type for this endpoint.
    type Res: DeserializeOwned + Serialize;

    /// The (inner) error type for this endpoint.
    type Err: DeserializeOwned + Serialize;

    /// The method used to make a request to the endpoint.
    const METHOD: Method;
}

/// Search endpoints.
pub mod search;

/// Animation endpoints.
pub mod animation;

/// Category endpoints.
pub mod category;

/// Image endpoints.
pub mod image;

/// Meta endpoints.
pub mod meta;

/// User endpoints.
pub mod user;

/// JIG endpoints.
pub mod jig;

/// Administrative endpoints.
pub mod admin;

/// Audio endpoints
pub mod audio;

/// Web Media library endpoints
pub mod media;

/// Session endpoints
pub mod session;

/// Locale endpoints
pub mod locale;

/// Pdf endpoints
pub mod pdf;

/// Course endpoints
pub mod course;

/// Additional Resource endpoints
pub mod additional_resource;

/// Resource endpoints
pub mod resource;

/// Circle endpoints
pub mod circle;

/// Module endpoints
pub mod module;

/// Item that can be part of PathParts
pub trait PathPart {
    /// string value to replace placeholder with
    fn get_path_string(&self) -> String;
}

/// Path of ApiEndpoint
pub trait PathParts {
    /// API path
    const PATH: &'static str;

    /// path path with placeholders replaced with values
    fn get_filled(&self) -> String;
}

// TODO: think we should try to get rid of all these impls, we should use NewTypes instead

impl PathPart for Uuid {
    fn get_path_string(&self) -> String {
        self.to_string()
    }
}

impl PathPart for Url {
    fn get_path_string(&self) -> String {
        // maybe use urlencoding crate
        todo!();
    }
}

impl PathPart for i32 {
    fn get_path_string(&self) -> String {
        self.to_string()
    }
}

impl PathPart for u32 {
    fn get_path_string(&self) -> String {
        self.to_string()
    }
}

impl PathPart for i16 {
    fn get_path_string(&self) -> String {
        self.to_string()
    }
}
