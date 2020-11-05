use crate::api::Method;
use serde::{de::DeserializeOwned, Serialize};

//  add something for path requests?
//  add something for auth required?

/// Represents a A endpoint that the backend will support, and how to call it.
pub trait ApiEndpoint {
    /// The request type for this endpoint.
    type Req: Serialize;

    /// The response type for this endpoint.
    type Res: DeserializeOwned + Serialize;

    /// The error type for this endpoint.
    type Err: DeserializeOwned + Serialize;

    /// The path to the endpoint.
    const PATH: &'static str;

    /// The method used to make a request to the endpoint.
    const METHOD: Method;
}

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

/// Module endpoints.
pub mod module;

/// Administrative endpoints.
pub mod admin;
