//! Types for categories.

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::endpoints::PathPart, domain::user::UserScope};

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a category.
    ///
    /// [`Uuid`]: ../../uuid/struct.Uuid.html
    pub struct CategoryId
}

#[derive(Serialize, Deserialize, Debug)]
/// The response returned when a request for categories is successful.
pub struct CategoryResponse {
    /// The categories returned.
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// The over-the-wire representation of a category.
pub struct Category {
    /// The category's id.
    pub id: CategoryId,

    /// The category's user_scope.
    pub user_scopes: Vec<UserScope>,

    /// The category's name.
    pub name: String,

    /// The category's children, if any.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Category>,

    /// When the category was initially created.
    pub created_at: DateTime<Utc>,

    /// When the category was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
/// When getting a tree of categories, which direction should the categories be followed?
pub enum CategoryTreeScope {
    /// Follow the parents up to the root.
    Ancestors,

    /// Follow the children down.
    Descendants,
}

#[derive(Serialize, Deserialize)]
/// Request to create a category.
pub struct CreateCategoryRequest {
    /// The name of the new category.
    pub name: String,

    /// The [`id`](Category::id) of the parent [`Category`](Category) to attatch it to.
    pub parent_id: Option<CategoryId>,
}

make_path_parts!(GetCategoryPath => "/v1/category");

/// Request to get a tree of categories.
///
/// # Examples
///
/// There are a few different use cases.
///
/// ### get root categories.
/// ```ignore
/// GetCategoryRequest { ids: vec![], scope: None }
/// ```
///
/// Additionally, you can do the same with `scope: Some(CategoryTreeScope::Ancestors)` but it is not considered the cannonical form.
///
/// ### get all categories
/// ```ignore
/// GetCategoryRequest { ids: vec![], scope: Some(CategoryTreeScope::Descendants) }
/// ```
///
/// ### get exact categories
/// ```ignore
/// GetCategoryRequest { ids: vec![id1, id2, ...], scope: None }
/// ```
///
/// ### get exact categories and their ancestors
/// ```ignore
/// GetCategoryRequest { ids: vec![id1, id2, ...], scope: Some(CategoryTreeScope::Ancestors) }
/// ```
///
/// ### get exact categories and their decendants.
/// ```ignore
/// GetCategoryRequest { ids: vec![id1, id2, ...], scope: Some(CategoryTreeScope::Descendants) }
/// ```
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetCategoryRequest {
    // fixme: Use CategoryId, unfortunately, sqlx doesn't currently allow for passing of T
    // the backend _could_ transmute the `CategoryId`s into `Uuid`s, but that's `unsafe`.
    /// The exact ids to be included in the response.
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    pub ids: Vec<Uuid>,

    /// Which direction to follow the tree.
    #[serde(default)]
    pub scope: Option<CategoryTreeScope>,
}

make_path_parts!(CreateCategoryPath => "/v1/category");

#[derive(Serialize, Deserialize)]
/// Response returned when a new category is created.
pub struct NewCategoryResponse {
    /// The offset visual offset into the parent category.
    pub index: u16,

    /// The id of the new category.
    pub id: CategoryId,
}

make_path_parts!(UpdateCategoryPath => "/v1/category/{}" => CategoryId);

#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
/// Request to update a category.
///
/// All fields are optional, any field that is [`None`] will not be updated.
///
/// # Errors
///
/// * [`UpdateError::OutOfRange`](crate::error::category::UpdateError::OutOfRange) if the given index is past the end of the parent.
pub struct UpdateCategoryRequest {
    /// If [`Some`] change the category's name to this name
    pub name: Option<String>,

    /// If [`Some`], change the parent to the given `Option<CategoryId>`.
    ///
    /// Specifically, if [`None`], don't update.
    /// If `Some(None)`, set the parent to [`None`].
    /// Otherwise set it to the given [`CategoryId`].
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<Option<CategoryId>>,

    /// If [`Some`] move to _before_ the category with the given index (ie, 0 moves to the start).
    ///
    /// # interactions
    /// If `index` is [`None`], and [`parent_id`](UpdateCategoryRequest::parent_id) is [`Some`] it will append to the end of the new parent.
    pub index: Option<u16>,

    /// If [`Some`] add a user scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scopes: Option<Vec<UserScope>>,
}

make_path_parts!(DeleteCategoryPath => "/v1/category/{}" => CategoryId);
