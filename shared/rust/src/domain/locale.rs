//! Locale types

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A bundle of [`Entry`]s
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The bundle's id
    pub id: Uuid,

    /// the bundle's name
    pub name: String,
}

/// What kind of item an [`Entry`] is.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemKind {
    /// The item kind's id
    pub id: Uuid,

    /// the item kind's name
    pub name: String,
}

/// The status of a given [`Entry`]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum EntryStatus {
    /// The entry has been approved.
    Approved = 0,

    /// The entry is in discussion.
    Discuss = 1,

    // todo: what does this even *mean*
    /// The entry is on hold.
    OnHold = 2,
}

// todo: an entry into the what?
/// An entry into the ?
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// This entry's id
    pub id: u32,

    /// This entry's parent [`Bundle`]'s id
    pub bundle_id: Uuid,

    /// The section this entry belongs in
    pub section: Option<String>,

    /// This entry's [`ItemKind`]'s id.
    pub item_kind_id: Option<Uuid>,

    /// The English version of this entry.
    pub english: Option<String>,

    /// The hebrew version of this entry.
    pub hebrew: Option<String>,

    /// This entry's current status.
    pub status: EntryStatus,

    /// A reference url in zeplin
    pub zeplin_reference: Option<String>,

    /// This entry's comments
    pub comments: Option<String>,

    /// If the entry is in the app.
    pub in_app: bool,

    /// If the entry is in an element.
    pub in_element: bool,

    /// If the entry is in mock.
    pub in_mock: bool,
}

/// Request for creating an entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryRequest {
    /// This entry's parent [`Bundle`]'s id
    pub bundle_id: Uuid,

    /// The section this entry belongs in
    pub section: Option<String>,

    /// This entry's [`ItemKind`]'s id.
    pub item_kind_id: Option<Uuid>,

    /// The English version of this entry.
    pub english: Option<String>,

    /// The hebrew version of this entry.
    pub hebrew: Option<String>,

    /// This entry's current status.
    pub status: EntryStatus,

    /// A reference url in zeplin
    pub zeplin_reference: Option<String>,

    /// This entry's comments
    pub comments: Option<String>,

    /// If the entry is in the app.
    pub in_app: bool,

    /// If the entry is in an element.
    pub in_element: bool,

    /// If the entry is in mock.
    pub in_mock: bool,
}

/// Response for successful creation of an entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryResponse {
    /// The newly created [`Entry`]'s id.
    pub id: u32,
}

/// Group by modifier for listing entries
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ListEntryGroupBy {
    /// No grouping, just return a plain list
    None,

    /// Group by parent bundle
    Bundle,
}

impl ListEntryGroupBy {
    /// Returns `true` if `self` is [`None`](Self::None).
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if `self` is [`Bundle`](Self::Bundle).
    pub fn is_bundle(&self) -> bool {
        matches!(self, Self::Bundle)
    }
}

impl Default for ListEntryGroupBy {
    fn default() -> Self {
        Self::None
    }
}

/// Query for listing [`entries`](Entry)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListEntryQuery {
    /// The [`Bundle`]s to filter to (empty means "all")
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(
        serialize_with = "crate::domain::csv_encode_uuids",
        deserialize_with = "crate::domain::from_csv"
    )]
    pub bundles: Vec<Uuid>,

    /// Whether the response should be returned as
    /// [`Bundles`](ListEntryResponse::Bundles) or [`List`](ListEntryResponse::List)
    #[serde(skip_serializing_if = "ListEntryGroupBy::is_none")]
    #[serde(default)]
    pub group_by: ListEntryGroupBy,
}

/// Response for listing entries
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ListEntryResponse {
    /// Entries grouped by [`Bundle`]
    Bundles(BTreeMap<Uuid, Vec<Entry>>),

    /// Ungrouped entries
    List(Vec<Entry>),
}

/// Response for getting a individual entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEntryResponse {
    /// The requested entry.
    pub entry: Entry,
}

/// Request for updating an [`Entry`]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntryRequest {
    /// This entry's parent [`Bundle`]'s id
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bundle_id: Option<Uuid>,

    /// The section this entry belongs in
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub section: Option<Option<String>>,

    /// This entry's [`ItemKind`]'s id.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub item_kind_id: Option<Uuid>,

    /// The English version of this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub english: Option<String>,

    /// The hebrew version of this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hebrew: Option<String>,

    /// This entry's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status: Option<EntryStatus>,

    /// A reference url in zeplin
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub zeplin_reference: Option<Option<String>>,

    /// This entry's comments
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comments: Option<Option<String>>,

    /// If the entry is in the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub in_app: Option<bool>,

    /// If the entry is in an element.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub in_element: Option<bool>,

    /// If the entry is in mock.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub in_mock: Option<bool>,
}

/// Response for listing bundles
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListBundleResponse {
    /// A list of bundles
    pub bundles: Vec<Bundle>,
}

/// Response for listing item kinds
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListItemKindResponse {
    /// A list of item kinds
    pub item_kinds: Vec<ItemKind>,
}
