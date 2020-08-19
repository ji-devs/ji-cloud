#[cfg(feature = "backend")]
use actix_web::{web, Route};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
}

#[cfg(feature = "backend")]
impl Method {
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
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Patch => "PATCH",
            Self::Post => "POST",
        }
    }
}