//! Endpoints and related.

/// A list of the endpoints that the server will accept.
pub mod endpoints;

/// http Method.
///
/// [_see `Method`_](enum.Method.html)
pub mod method;

#[allow(missing_docs)]
#[deprecated]
pub mod result;

pub use endpoints::ApiEndpoint;
pub use method::Method;
