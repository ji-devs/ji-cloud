use self::meta::{AffiliationId, AgeRangeId, StyleId};
use super::category::CategoryId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
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
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
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

// HACK: we can't get `Vec<_>` directly from the DB, so we have to work around it for now.
// see: https://github.com/launchbadge/sqlx/issues/298
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for Image {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbImage {
            id,
            name,
            description,
            is_premium,
            publish_at,
            styles,
            age_ranges,
            affiliations,
            categories,
            created_at,
            updated_at,
        } = DbImage::from_row(row)?;

        Ok(Self {
            id,
            name,
            description,
            is_premium,
            publish_at,
            styles: styles.into_iter().map(|(it,)| it).collect(),
            age_ranges: age_ranges.into_iter().map(|(it,)| it).collect(),
            affiliations: affiliations.into_iter().map(|(it,)| it).collect(),
            categories: categories.into_iter().map(|(it,)| it).collect(),
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbImage {
    pub id: ImageId,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<(StyleId,)>,
    pub age_ranges: Vec<(AgeRangeId,)>,
    pub affiliations: Vec<(AffiliationId,)>,
    pub categories: Vec<(CategoryId,)>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub id: ImageId,
    pub upload_url: Url,
}
