//! Types for metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`], represents the ID of a [`Style`].
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
/// [`Style`]: struct.Style.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct StyleId(pub Uuid);

/// Wrapper type around [`Uuid`], represents the ID of a [`AgeRange`].
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
/// [`AgeRange`]: struct.AgeRange.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AgeRangeId(pub Uuid);

/// Wrapper type around [`Uuid`], represents the ID of a [`Affiliation`].
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
/// [`Affiliation`]: struct.Affiliation.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AffiliationId(pub Uuid);

/// Wrapper type around [`Uuid`], represents the ID of a [`Subject`].
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
/// [`Subject`]: struct.Subject.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct SubjectId(pub Uuid);

into_uuid!(StyleId, AffiliationId, AgeRangeId, SubjectId);

/// Represents a style.
#[derive(Serialize, Deserialize, Debug)]
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

/// Response for fetching all metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    /// All styles the server has.
    pub styles: Vec<Style>,

    /// All age ranges the server has.
    pub age_ranges: Vec<AgeRange>,

    /// All affiliations the server has.
    pub affiliations: Vec<Affiliation>,

    /// All subjects the server has.
    pub subjects: Vec<Subject>,
}

/// Metadata kinds.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum MetaKind {
    /// [`Affiliation`]
    ///
    /// [`Affiliation`]: struct.Affiliation.html
    Affiliation,

    /// [`Style`]
    ///
    /// [`Style`]: struct.Style.html
    Style,

    /// [`AgeRange`]
    ///
    /// [`AgeRange`]: struct.AgeRange.html
    AgeRange,

    /// [`Category`]
    ///
    /// [`Category`]: struct.Category.html
    Category,

    /// [`Subject`]
    ///
    /// [`Subject`]: struct.Subject.html
    Subject,
}
