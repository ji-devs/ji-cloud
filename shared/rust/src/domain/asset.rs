//! Types for Assets, Jig and LearningPath.

use std::{
    collections::HashMap,
    fmt::{self, Debug},
    str::FromStr,
};

use chrono::{DateTime, Utc};
// use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    category::CategoryId,
    // learning_path::AdditionalResource,
    meta::{AffiliationId, AgeRangeId},
};

use super::{
    course::{CourseId, CourseResponse},
    jig::{JigId, JigResponse, LiteModule},
};

pub mod additional_resource;

/// AssetId
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AssetId {
    /// JIG ID
    JigId(JigId),

    /// Course ID
    CourseId(CourseId),
}

impl From<JigId> for AssetId {
    fn from(jig_id: JigId) -> Self {
        Self::JigId(jig_id)
    }
}

impl From<CourseId> for AssetId {
    fn from(course_id: CourseId) -> Self {
        Self::CourseId(course_id)
    }
}

impl AssetId {
    /// get jig id value as ref
    pub fn unwrap_jig(&self) -> &JigId {
        match self {
            Self::JigId(jig_id) => jig_id,
            _ => panic!(),
        }
    }

    /// get course id value as ref
    pub fn unwrap_course(&self) -> &CourseId {
        match self {
            Self::CourseId(course_id) => course_id,
            _ => panic!(),
        }
    }

    /// get the id uuid
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::JigId(jig_id) => &jig_id.0,
            Self::CourseId(course_id) => &course_id.0,
        }
    }
}

/// Asset
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Asset {
    /// JIG ID associated with the module.
    Jig(JigResponse),

    /// Course ID associated with the module.
    Course(CourseResponse),
}

impl From<JigResponse> for Asset {
    fn from(jig: JigResponse) -> Self {
        Self::Jig(jig)
    }
}

impl From<CourseResponse> for Asset {
    fn from(course: CourseResponse) -> Self {
        Self::Course(course)
    }
}

impl Asset {
    /// get jig value as ref
    pub fn unwrap_jig(&self) -> &JigResponse {
        match self {
            Self::Jig(jig) => jig,
            _ => panic!(),
        }
    }

    /// get course value as ref
    pub fn unwrap_course(&self) -> &CourseResponse {
        match self {
            Self::Course(course) => course,
            _ => panic!(),
        }
    }

    /// get id
    pub fn id(&self) -> AssetId {
        match self {
            Self::Jig(jig) => jig.id.into(),
            Self::Course(course) => course.id.into(),
        }
    }

    /// get id
    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Jig(jig) => jig.published_at,
            Self::Course(course) => course.published_at,
        }
    }

    /// get display_name
    pub fn display_name(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.display_name,
            Self::Course(course) => &course.course_data.display_name,
        }
    }

    /// get language
    pub fn language(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.language,
            Self::Course(course) => &course.course_data.language,
        }
    }

    /// get description
    pub fn description(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.description,
            Self::Course(course) => &course.course_data.description,
        }
    }

    /// get cover
    pub fn cover(&self) -> Option<&LiteModule> {
        match self {
            Self::Jig(jig) => jig.jig_data.modules.first(),
            Self::Course(_) => todo!(),
        }
    }

    /// get privacy_level
    pub fn privacy_level(&self) -> &PrivacyLevel {
        match self {
            Self::Jig(jig) => &jig.jig_data.privacy_level,
            Self::Course(course) => &course.course_data.privacy_level,
        }
    }

    /// get other_keywords
    pub fn other_keywords(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.other_keywords,
            Self::Course(course) => &course.course_data.other_keywords,
        }
    }

    /// get translated_keywords
    pub fn translated_keywords(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.translated_keywords,
            Self::Course(course) => &course.course_data.translated_keywords,
        }
    }

    /// get age_ranges
    pub fn age_ranges(&self) -> &Vec<AgeRangeId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.age_ranges,
            Self::Course(course) => &course.course_data.age_ranges,
        }
    }

    /// get affiliations
    pub fn affiliations(&self) -> &Vec<AffiliationId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.affiliations,
            Self::Course(course) => &course.course_data.affiliations,
        }
    }

    /// get categories
    pub fn categories(&self) -> &Vec<CategoryId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.categories,
            Self::Course(course) => &course.course_data.categories,
        }
    }

    // pub fn additional_resources(&self) -> &Vec<AdditionalResource> {
    //     match self {
    //         Self::Jig(_) => todo!(),
    //         Self::Course(_) => todo!(),
    //     }
    // }

    /// get translated_description
    pub fn translated_description(&self) -> &HashMap<String, String> {
        match self {
            Self::Jig(jig) => &jig.jig_data.translated_description,
            Self::Course(course) => &course.course_data.translated_description,
        }
    }
}

// dyn_clone::clone_trait_object!(Asset);

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

/// Access level for the jig.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum PrivacyLevel {
    /// Publicly available and indexed. Can be shared with others.
    Public = 0,

    /// Not indexed, but can be accessed by non-owners if the id is known. "Private" in the front-end
    Unlisted = 1,

    /// NOT IMPLEMENTED. Only available to the author.
    Private = 2,
}

impl PrivacyLevel {
    /// Represents the privacy level as a `str`. Relevant for Algolia tag filtering.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Unlisted => "unlisted",
            Self::Private => "private",
        }
    }
}

impl Default for PrivacyLevel {
    fn default() -> Self {
        Self::Public
    }
}

/// Whether the data is draft or live.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum DraftOrLive {
    /// Represents a draft copy
    Draft = 0,
    /// Represents a live copy
    Live = 1,
}

impl DraftOrLive {
    /// Returns `true` for a [`Self::Live`] value.
    ///
    /// ```
    /// let x = DraftOrLive::Live;
    /// assert_eq!(x.is_live(), true);
    ///
    /// let x = DraftOrLive::Draft;
    /// assert_eq!(x.is_live(), false);
    /// ```
    pub fn is_live(&self) -> bool {
        matches!(*self, DraftOrLive::Live)
    }

    /// Returns `true` for a [`Draft`] value.
    ///
    /// ```
    /// let x = DraftOrLive::Live;
    /// assert_eq!(x.is_draft(), false);
    ///
    /// let x = DraftOrLive::Draft;
    /// assert_eq!(x.is_draft(), true);
    /// ```
    pub fn is_draft(&self) -> bool {
        !self.is_live()
    }
}

impl From<DraftOrLive> for bool {
    fn from(draft_or_live: DraftOrLive) -> Self {
        match draft_or_live {
            DraftOrLive::Draft => false,
            DraftOrLive::Live => true,
        }
    }
}

impl From<bool> for DraftOrLive {
    fn from(draft_or_live: bool) -> Self {
        match draft_or_live {
            false => DraftOrLive::Draft,
            true => DraftOrLive::Live,
        }
    }
}
