//! Types for JIGs.

pub mod additional_resource;
pub use additional_resource::AdditionalResourceId;

pub mod module;
// avoid breaking Changes
pub use module::{LiteModule, Module, ModuleKind};

pub mod player;
pub use player::{JigPlayerSettings, TextDirection};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, str::FromStr};
use uuid::Uuid;

use super::{
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, GoalId},
    Publish,
};
use crate::domain::jig::module::body::ThemeId;

/// Wrapper type around [`Uuid`], represents the ID of a JIG.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigId(pub Uuid);

/// Special parameter for allowing implicit `me` as a user.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UserOrMe {
    /// We should use the user found in the session auth.
    Me,

    /// we should use the provided user.
    User(Uuid),
}

impl serde::Serialize for UserOrMe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UserOrMe::Me => serializer.serialize_str("me"),
            UserOrMe::User(id) => serializer.collect_str(&id.to_hyphenated()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserOrMe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserOrMe;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("`me` or `<uuid>`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value == "me" {
                    Ok(UserOrMe::Me)
                } else {
                    Uuid::from_str(value)
                        .map(UserOrMe::User)
                        .map_err(|e| E::custom(format!("failed to parse id: {}", e)))
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// Request to create a new JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JigCreateRequest {
    /// The JIG's name.
    #[serde(default)]
    pub display_name: String,

    /// The goals of this jig.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub goals: Vec<GoalId>,

    /// This jig's age ranges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub age_ranges: Vec<AgeRangeId>,

    /// This jig's affiliations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub affiliations: Vec<AffiliationId>,

    /// The language the jig uses.
    ///
    /// If None, uses the user's language.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: Option<String>,

    /// The jig's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,

    /// When the JIG should be considered published (if at all).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Publish>,

    /// Description of the jig. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// Default player settings for this jig.
    #[serde(default)]
    pub default_player_settings: JigPlayerSettings,
}

/// The over-the-wire representation of a JIG.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Jig {
    /// The ID of the JIG.
    pub id: JigId,

    /// The JIG's name.
    pub display_name: String,

    /// The JIG's remaining modules.
    pub modules: Vec<LiteModule>,

    /// This jig's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// This jig's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The goals of this jig.
    pub goals: Vec<GoalId>,

    /// The ID of the JIG's original creator ([`None`] if unknown).
    pub creator_id: Option<Uuid>,

    /// The current author
    pub author_id: Option<Uuid>,

    /// The language the jig uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// The jig's categories.
    pub categories: Vec<CategoryId>,

    /// When the JIG should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// Additional resources of this JIG.
    pub additional_resources: Vec<AdditionalResourceId>,

    /// Description of the jig.
    pub description: String,

    /// When the jig was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// Whether the jig is public or not.
    pub is_public: bool,

    /// Default player settings for this jig.
    pub default_player_settings: JigPlayerSettings,

    /// Theme for this jig, identified by `[ThemeId](jig::module::body::ThemeId)`.
    pub theme: ThemeId,

    /// Background audio
    pub audio_background: Option<AudioBackground>,

    /// Audio effects
    pub audio_effects: AudioEffects,
}

/// Audio for background music
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AudioBackground {
    #[allow(missing_docs)]
    FunForKids = 0,
    #[allow(missing_docs)]
    SukkotLoop = 1,
}
impl AudioBackground {
    /// Get all enum variants
    pub fn variants() -> Vec<AudioBackground> {
        vec![AudioBackground::FunForKids, AudioBackground::SukkotLoop]
    }
}

/// Audio Effects
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AudioEffects {
    /// Positive audio feedback
    pub feedback_positive: HashSet<AudioFeedbackPositive>,

    /// Negative audio feedback
    pub feedback_negative: HashSet<AudioFeedbackNegative>,
}

/// Negative Audio Feedback
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AudioFeedbackNegative {
    #[allow(missing_docs)]
    Boing = 0,
    #[allow(missing_docs)]
    Incorrect = 1,
    #[allow(missing_docs)]
    JumpWrong = 2,
}
impl AudioFeedbackNegative {
    /// Get all enum variants
    pub fn variants() -> Vec<AudioFeedbackNegative> {
        vec![
            AudioFeedbackNegative::Boing,
            AudioFeedbackNegative::Incorrect,
            AudioFeedbackNegative::JumpWrong,
        ]
    }
}

/// Positive Audio Feedback
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AudioFeedbackPositive {
    #[allow(missing_docs)]
    Magic = 0,
    #[allow(missing_docs)]
    StarPing = 1,
    #[allow(missing_docs)]
    Ting = 2,
}
impl AudioFeedbackPositive {
    /// Get all enum variants
    pub fn variants() -> Vec<AudioFeedbackPositive> {
        vec![
            AudioFeedbackPositive::Magic,
            AudioFeedbackPositive::StarPing,
            AudioFeedbackPositive::Ting,
        ]
    }
}

/// The response returned when a request for `GET`ing a jig is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigResponse {
    /// The requested JIG.
    pub jig: Jig,
}

/// Request for updating a JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JigUpdateRequest {
    /// The JIG's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The language the jig uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,

    /// The jig's categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// The goals of this jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub goals: Option<Vec<GoalId>>,

    /// The jig's age ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// The jig's affiliations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<Uuid>,

    /// When the JIG should be considered published (if at all).
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<Publish>>,

    /// Additional resources of this JIG.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub additional_resources: Option<Vec<AdditionalResourceId>>,

    /// Description of the jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Whether the jig is public or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_public: Option<bool>,

    /// Default player settings for this jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_player_settings: Option<JigPlayerSettings>,

    /// Theme for this jig, identified by `[ThemeId](jig::module::body::ThemeId)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub theme: Option<ThemeId>,

    /// Background audio
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audio_background: Option<Option<AudioBackground>>,

    /// Audio effects
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audio_effects: Option<AudioEffects>,
}

/// Query for [`Browse`](crate::api::endpoints::jig::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the jigs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Response for [`Browse`](crate::api::endpoints::jig::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigBrowseResponse {
    /// the jigs returned.
    pub jigs: Vec<Jig>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of jigs found
    pub total_jig_count: u64,
}

/// Search for jigs via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JigSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the jigs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `age_ranges`
    ///
    /// Note: Currently does nothing
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    /// Optionally filter by `affiliations`
    ///
    /// Note: Currently does nothing
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    /// Optionally filter by `categories`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,

    /// Optionally filter by `goals`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub goals: Vec<GoalId>,

    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by the author
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Uuid>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JigSearchResponse {
    /// the jigs returned.
    pub jigs: Vec<JigResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of jigs found
    pub total_jig_count: u64,
}

/// Response for successfully finding the draft of a jig.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JigDraftResponse {
    /// The ID of the jig
    pub id: JigId,
}

/// Response for total count of public and published jig.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JigCountResponse {
    /// Total number of public and published jigs.
    pub total_count: u64,
}

into_uuid![JigId];
