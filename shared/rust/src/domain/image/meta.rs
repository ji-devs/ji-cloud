use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct StyleId(pub Uuid);

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AgeRangeId(pub Uuid);

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AffiliationId(pub Uuid);

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    pub id: StyleId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgeRange {
    pub id: AgeRangeId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Affiliation {
    pub id: AffiliationId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleResponse {
    pub styles: Vec<Style>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgeRangeResponse {
    pub age_ranges: Vec<AgeRange>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffiliationResponse {
    pub affiliations: Vec<Affiliation>,
}
