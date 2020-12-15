//! Types for metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;

/// Wrapper type around [`Uuid`], represents [`Style::id`].
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct StyleId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`AgeRange::id`].
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AgeRangeId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Affiliation::id`].
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AffiliationId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`Subject::id`].
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct SubjectId(pub Uuid);

/// Wrapper type around [`Uuid`], represents [`ContentType::id`].
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ContentTypeId(pub Uuid);

into_uuid!(StyleId, AffiliationId, AgeRangeId, SubjectId, ContentTypeId);

/// Represents a style.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Style {
    /// The id of the style.
    pub id: StyleId,

    /// The style's name.
    pub display_name: String,

    /// When the style was created.
    pub created_at: DateTime<Utc>,

    /// When the style was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a age range.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AgeRange {
    /// The id of the age range.
    pub id: AgeRangeId,

    /// The age range's name.
    pub display_name: String,

    /// When the age range was created.
    pub created_at: DateTime<Utc>,

    /// When the age range was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents an affiliation.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Affiliation {
    /// The id of the affiliation.
    pub id: AffiliationId,

    /// The affiliation's name.
    pub display_name: String,

    /// When the affiliation was created.
    pub created_at: DateTime<Utc>,

    /// When the affiliation was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a subject.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Subject {
    /// The id of the subject.
    pub id: SubjectId,

    /// The subject's name.
    pub display_name: String,

    /// When the subject was created.
    pub created_at: DateTime<Utc>,

    /// When the subject was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a content-type.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ContentType {
    /// The id of the content-type.
    pub id: ContentTypeId,

    /// The content-type's name.
    pub display_name: String,

    /// When the content-type was created.
    pub created_at: DateTime<Utc>,

    /// When the content-type was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for fetching all metadata.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct MetadataResponse {
    /// All styles the server has.
    pub styles: Vec<Style>,

    /// All age ranges the server has.
    pub age_ranges: Vec<AgeRange>,

    /// All affiliations the server has.
    pub affiliations: Vec<Affiliation>,

    /// All subjects the server has.
    pub subjects: Vec<Subject>,

    /// All content types
    pub content_types: Vec<ContentType>,
}

/// Metadata kinds.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum MetaKind {
    /// [`Affiliation`]
    Affiliation,

    /// [`Style`]
    Style,

    /// [`AgeRange`]
    AgeRange,

    /// [`Category`](super::category::Category)
    Category,

    /// [`Subject`]
    Subject,

    /// [`ContentType`]
    ContentType,
}
