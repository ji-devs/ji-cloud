use self::meta::{AffiliationId, AgeRangeId, StyleId};
use super::category::CategoryId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub mod meta;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ImageId(pub Uuid);

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRequest {
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<StyleId>,
    pub age_ranges: Vec<AgeRangeId>,
    pub affiliations: Vec<AffiliationId>,
    pub categories: Vec<CategoryId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_premium: Option<bool>,
    // Note that once `publish_at` on the resource is in the past, it becomes immutable
    //  (which means that setting this will do nothing)
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<DateTime<Utc>>>,
    pub styles: Option<Vec<StyleId>>,
    pub age_ranges: Option<Vec<AgeRangeId>>,
    pub affiliations: Option<Vec<AffiliationId>>,
    pub categories: Option<Vec<CategoryId>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    pub id: ImageId,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<StyleId>,
    pub age_ranges: Vec<AgeRangeId>,
    pub affiliations: Vec<AffiliationId>,
    pub categories: Vec<CategoryId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub id: ImageId,
    pub upload_url: Url,
}
