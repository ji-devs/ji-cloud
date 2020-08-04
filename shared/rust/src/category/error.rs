#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
macro_rules! from_anyhow {
    ( $( $t:ty ),+ ) => {
        $(
            impl<T: Into<anyhow::Error>> From<T> for $t {
                fn from(e: T) -> Self {
                    Self::InternalServerError(e.into())
                }
            }
        )+
    };
}

#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    resp.into()
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryGetError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<CategoryGetError> for actix_web::Error {
    fn from(e: CategoryGetError) -> actix_web::Error {
        match e {
            CategoryGetError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryGetError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryCreateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    Forbidden,
    ParentCategoryNotFound,
}

#[cfg(feature = "backend")]
impl From<CategoryCreateError> for actix_web::Error {
    fn from(e: CategoryCreateError) -> actix_web::Error {
        match e {
            CategoryCreateError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryCreateError::ParentCategoryNotFound => HttpResponse::NotFound().into(),
            CategoryCreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryDeleteError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    CategoryNotFound,
    Forbidden,
    // todo: should the IDs of the children be here?
    Children,
}

#[cfg(feature = "backend")]
impl From<CategoryDeleteError> for actix_web::Error {
    fn from(e: CategoryDeleteError) -> actix_web::Error {
        match e {
            CategoryDeleteError::InternalServerError(e) => anyhow_to_ise(e),
            CategoryDeleteError::CategoryNotFound => HttpResponse::NotFound().into(),
            CategoryDeleteError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryUpdateError {
    #[serde(skip)]
    InternalServerError(anyhow::Error),
    CategoryNotFound,
    ParentCategoryNotFound,
    Forbidden,
    Cycle,
    OutOfRange {
        max: u16,
    },
}

#[cfg(feature = "backend")]
impl From<CategoryUpdateError> for actix_web::Error {
    fn from(e: CategoryUpdateError) -> actix_web::Error {
        match e {
            CategoryUpdateError::InternalServerError(e) => anyhow_to_ise(e),
            e @ CategoryUpdateError::CategoryNotFound
            | e @ CategoryUpdateError::ParentCategoryNotFound => {
                HttpResponse::NotFound().json(e).into()
            }
            CategoryUpdateError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[cfg(feature = "backend")]
from_anyhow![
    CategoryGetError,
    CategoryCreateError,
    CategoryDeleteError,
    CategoryUpdateError
];
