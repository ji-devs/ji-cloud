#[cfg(feature = "backend")]
use actix_web::HttpResponse;
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct CategoryId(pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub parent_id: Option<CategoryId>,
    pub name: String,
    pub id: CategoryId,
    pub index: u16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for Category {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbCategory {
            parent_id,
            name,
            id,
            index,
            created_at,
            updated_at,
        } = DbCategory::from_row(row)?;

        Ok(Category {
            parent_id,
            name,
            id,
            index: index as u16,
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbCategory {
    pub parent_id: Option<CategoryId>,
    pub name: String,
    pub id: CategoryId,
    pub index: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
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
    pub name: String,
    pub parent_id: Option<CategoryId>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCategoryResponse {
    pub index: u16,
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
    Cycle,
    OutOfRange { max: u16 },
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

#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    /// If None, don't change parents. If Some, change parent to the given CategoryId (or null).
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<Option<CategoryId>>,
    /// If None, don't change index. If Some move to _before_ whatever has the given index (ie, 0 moves to the start).
    /// Will cause an error if you try to move to past the end of the parent.
    ///
    /// If None and parent_id is Some(...) it will append to the end of the new parent.
    pub index: Option<u16>,
}

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
