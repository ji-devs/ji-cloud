#[cfg(feature = "backend")]
use paperclip::actix::web::{self, Route};

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

    /// http `PUT`, used to replace resources.
    Put,
}

#[cfg(feature = "backend")]
impl Method {
    /// Gets a [`Route`](Route) based off of `Self`.
    pub fn route(self) -> Route {
        match self {
            Self::Delete => web::delete(),
            Self::Get => web::get(),
            Self::Patch => web::patch(),
            Self::Post => web::post(),
            Self::Put => web::put(),
        }
    }
}

impl Method {
    /// Represents `Self` as a [`str`](str).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
        }
    }
}
