#[cfg(feature = "backend")]
use actix_web::{web, Route};

/// Represents a http method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Method {
    /// http `DELETE`, used for deleting resources.
    Delete,

    /// http `GET`, used to retrieving resources.
    Get,

    /// http `PATCH`, used to update resources.
    Patch,

    /// http `POST`, used to create resources.
    Post,
}

#[cfg(feature = "backend")]
impl Method {
    /// Gets a [`actix_web::Route`] based off of `Self`.
    ///
    /// [`actix_web::Route`]: ../../../actix_web/struct.Route.html
    pub fn route(self) -> Route {
        match self {
            Self::Delete => web::delete(),
            Self::Get => web::get(),
            Self::Patch => web::patch(),
            Self::Post => web::post(),
        }
    }
}

impl Method {
    /// Represents `Self` as a [`str`].
    ///
    /// [`str`]: https://doc.rust-lang.org/stable/std/primitive.str.html
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Patch => "PATCH",
            Self::Post => "POST",
        }
    }
}
