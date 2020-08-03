#[cfg(feature = "backend")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CategoryId(pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    pub tree: Category,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub children: Vec<Category>,
    pub name: String,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryGetError {
    InternalServerError,
    Forbidden,
}

#[cfg(feature = "backend")]
impl From<CategoryGetError> for actix_web::Error {
    fn from(e: CategoryGetError) -> actix_web::Error {
        match e {
            CategoryGetError::InternalServerError => HttpResponse::InternalServerError().into(),
            CategoryGetError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub index: Option<usize>,
    pub name: String,
    pub parent: Option<CategoryId>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCategoryResponse {
    pub index: usize,
    pub id: CategoryId,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryCreateError {
    InternalServerError,
    Forbidden,
    ParentCategoryNotFound,
}

#[cfg(feature = "backend")]
impl From<CategoryCreateError> for actix_web::Error {
    fn from(e: CategoryCreateError) -> actix_web::Error {
        match e {
            CategoryCreateError::InternalServerError => HttpResponse::InternalServerError().into(),
            CategoryCreateError::ParentCategoryNotFound => HttpResponse::NotFound().into(),
            CategoryCreateError::Forbidden => HttpResponse::Forbidden().into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryDeleteError {
    InternalServerError,
    CategoryNotFound,
    Forbidden,
    // todo: should the IDs of the children be here?
    Children,
}

#[cfg(feature = "backend")]
impl From<CategoryDeleteError> for actix_web::Error {
    fn from(e: CategoryDeleteError) -> actix_web::Error {
        match e {
            CategoryDeleteError::InternalServerError => HttpResponse::InternalServerError().into(),
            CategoryDeleteError::CategoryNotFound => HttpResponse::NotFound().into(),
            CategoryDeleteError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub enum CategoryUpdateError {
    InternalServerError,
    CategoryNotFound,
    ParentCategoryNotFound,
    Forbidden,
    OutOfRange { max: usize },
}

#[cfg(feature = "backend")]
impl From<CategoryUpdateError> for actix_web::Error {
    fn from(e: CategoryUpdateError) -> actix_web::Error {
        match e {
            CategoryUpdateError::InternalServerError => HttpResponse::InternalServerError().into(),
            e @ CategoryUpdateError::CategoryNotFound
            | e @ CategoryUpdateError::ParentCategoryNotFound => {
                HttpResponse::NotFound().json(e).into()
            }
            CategoryUpdateError::Forbidden => HttpResponse::Forbidden().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CategoryUpdateRequest {
    pub name: Option<String>,
    /// If None, don't change parents. If Some, change parent to the given CategoryId (or null).
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<CategoryId>>,
    /// If None, don't change index. If Some move to _before_ whatever has the given index (ie, 0 moves to the start).
    /// Will cause an error if you try to move to past the end of the parent.
    ///
    /// If None and parent_id is Some(...) it will append to the end of the new parent.
    pub index: Option<usize>,
}

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
