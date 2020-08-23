use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct CategoryId(pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Category>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub image_count: u64,
    pub jig_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CategoryTreeScope {
    Ancestors,
    Decendants,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub parent_id: Option<CategoryId>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetCategoryRequest {
    // fixme: Use CategoryId, unfortunately, sqlx doesn't currently allow for passing of T
    // the backend _could_ transmute the `CategoryId`s into `Uuid`s, but that's `unsafe`.
    #[serde(default)]
    pub ids: Vec<Uuid>,
    #[serde(default)]
    pub scope: Option<CategoryTreeScope>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCategoryResponse {
    pub index: u16,
    pub id: CategoryId,
}

#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    /// If None, don't change parents. If Some, change parent to the given CategoryId (or null).
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<Option<CategoryId>>,
    /// If None, don't change index. If Some move to _before_ whatever has the given index (ie, 0 moves to the start).
    /// Will cause an error if you try to move to past the end of the parent.
    ///
    /// If None and parent_id is Some(...) it will append to the end of the new parent.
    pub index: Option<u16>,
}
