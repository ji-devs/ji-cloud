//! Types for Assets, Jig and LearningPath.

use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::{self, Debug},
    str::FromStr,
};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
// use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;

use crate::{
    api::endpoints::PathPart,
    domain::{
        category::CategoryId,
        meta::{AffiliationId, AgeRangeId},
        module::LiteModule,
    },
};

use super::{
    additional_resource::AdditionalResource,
    course::{CourseId, CourseResponse},
    jig::{JigId, JigResponse},
    module::body::ThemeId,
    resource::{ResourceId, ResourceResponse},
    user::UserId,
};

/// AssetType
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "snake_case")]
pub enum AssetType {
    /// JIG
    Jig,

    /// Resource
    Resource,

    /// Course
    Course,
}

impl AssetType {
    /// check if jig
    pub fn is_jig(&self) -> bool {
        matches!(self, Self::Jig)
    }

    /// check if resource
    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource)
    }

    /// check if course
    pub fn is_course(&self) -> bool {
        matches!(self, Self::Course)
    }

    /// Represents the asset type as a `str`
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Jig => "jig",
            Self::Resource => "resource",
            Self::Course => "course",
        }
    }

    /// Create asset id from self and uuid
    pub fn to_asset_id(&self, uuid: Uuid) -> AssetId {
        match self {
            AssetType::Jig => JigId(uuid).into(),
            AssetType::Course => CourseId(uuid).into(),
            AssetType::Resource => ResourceId(uuid).into(),
        }
    }
}

impl From<&AssetId> for AssetType {
    fn from(asset_id: &AssetId) -> Self {
        match asset_id {
            AssetId::JigId(_) => AssetType::Jig,
            AssetId::CourseId(_) => AssetType::Course,
            AssetId::ResourceId(_) => AssetType::Resource,
        }
    }
}

impl TryFrom<&str> for AssetType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "jig" => Ok(Self::Jig),
            "course" => Ok(Self::Course),
            "resource" => Ok(Self::Resource),
            _ => Err(()),
        }
    }
}

impl PathPart for AssetType {
    fn get_path_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// AssetId
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AssetId {
    /// JIG ID
    JigId(JigId),

    /// Course ID
    CourseId(CourseId),

    /// Resource ID
    ResourceId(ResourceId),
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

impl From<ResourceId> for AssetId {
    fn from(resource_id: ResourceId) -> Self {
        Self::ResourceId(resource_id)
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

    /// get resource id value as ref
    pub fn unwrap_resource(&self) -> &ResourceId {
        match self {
            Self::ResourceId(resource_id) => resource_id,
            _ => panic!(),
        }
    }

    /// get the id uuid
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::JigId(jig_id) => &jig_id.0,
            Self::CourseId(course_id) => &course_id.0,
            Self::ResourceId(resource_id) => &resource_id.0,
        }
    }

    /// check if jig
    pub fn is_jig_id(&self) -> bool {
        matches!(self, Self::JigId(_))
    }

    /// check if course
    pub fn is_course_id(&self) -> bool {
        matches!(self, Self::CourseId(_))
    }

    /// check if resource
    pub fn is_resource_id(&self) -> bool {
        matches!(self, Self::ResourceId(_))
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

    /// Resource ID associated with the module.
    Resource(ResourceResponse),
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

impl From<ResourceResponse> for Asset {
    fn from(resource: ResourceResponse) -> Self {
        Self::Resource(resource)
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

    /// get resource value as ref
    pub fn unwrap_resource(&self) -> &ResourceResponse {
        match self {
            Self::Resource(resource) => resource,
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

    /// check if is jig
    pub fn is_jig(&self) -> bool {
        matches!(self, Self::Jig(_))
    }

    /// check if is course
    pub fn is_course(&self) -> bool {
        matches!(self, Self::Course(_))
    }

    /// check if is resource
    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    /// get id
    pub fn id(&self) -> AssetId {
        match self {
            Self::Jig(jig) => jig.id.into(),
            Self::Course(course) => course.id.into(),
            Self::Resource(resource) => resource.id.into(),
        }
    }

    /// get id
    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Jig(jig) => jig.published_at,
            Self::Course(course) => course.published_at,
            Self::Resource(resource) => resource.published_at,
        }
    }

    /// get display_name
    pub fn display_name(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.display_name,
            Self::Course(course) => &course.course_data.display_name,
            Self::Resource(resource) => &resource.resource_data.display_name,
        }
    }

    /// get language
    pub fn language(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.language,
            Self::Course(course) => &course.course_data.language,
            Self::Resource(resource) => &resource.resource_data.language,
        }
    }

    /// get description
    pub fn description(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.description,
            Self::Course(course) => &course.course_data.description,
            Self::Resource(resource) => &resource.resource_data.description,
        }
    }

    /// get cover
    pub fn cover(&self) -> Option<&LiteModule> {
        match self {
            Self::Jig(jig) => jig.jig_data.modules.first(),
            Self::Course(course) => course.course_data.cover.as_ref(),
            Self::Resource(resource) => resource.resource_data.cover.as_ref(),
        }
    }

    /// get privacy_level
    pub fn privacy_level(&self) -> &PrivacyLevel {
        match self {
            Self::Jig(jig) => &jig.jig_data.privacy_level,
            Self::Course(course) => &course.course_data.privacy_level,
            Self::Resource(resource) => &resource.resource_data.privacy_level,
        }
    }

    /// get other_keywords
    pub fn other_keywords(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.other_keywords,
            Self::Course(course) => &course.course_data.other_keywords,
            Self::Resource(resource) => &resource.resource_data.other_keywords,
        }
    }

    /// get translated_keywords
    pub fn translated_keywords(&self) -> &String {
        match self {
            Self::Jig(jig) => &jig.jig_data.translated_keywords,
            Self::Course(course) => &course.course_data.translated_keywords,
            Self::Resource(resource) => &resource.resource_data.translated_keywords,
        }
    }

    /// get age_ranges
    pub fn age_ranges(&self) -> &Vec<AgeRangeId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.age_ranges,
            Self::Course(course) => &course.course_data.age_ranges,
            Self::Resource(resource) => &resource.resource_data.age_ranges,
        }
    }

    /// get affiliations
    pub fn affiliations(&self) -> &Vec<AffiliationId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.affiliations,
            Self::Course(course) => &course.course_data.affiliations,
            Self::Resource(resource) => &resource.resource_data.affiliations,
        }
    }

    /// get categories
    pub fn categories(&self) -> &Vec<CategoryId> {
        match self {
            Self::Jig(jig) => &jig.jig_data.categories,
            Self::Course(course) => &course.course_data.categories,
            Self::Resource(resource) => &resource.resource_data.categories,
        }
    }

    /// get likes
    pub fn likes(&self) -> i64 {
        match self {
            Self::Jig(jig) => jig.likes,
            Self::Course(course) => course.likes,
            Self::Resource(resource) => resource.likes,
        }
    }

    /// get plays
    pub fn plays(&self) -> i64 {
        match self {
            Self::Jig(jig) => jig.plays,
            Self::Course(course) => course.plays,
            Self::Resource(resource) => resource.views,
        }
    }

    /// get author_id
    pub fn author_id(&self) -> &Option<UserId> {
        match self {
            Self::Jig(jig) => &jig.author_id,
            Self::Course(course) => &course.author_id,
            Self::Resource(resource) => &resource.author_id,
        }
    }

    /// get author_name
    pub fn author_name(&self) -> &Option<String> {
        match self {
            Self::Jig(jig) => &jig.author_name,
            Self::Course(course) => &course.author_name,
            Self::Resource(resource) => &resource.author_name,
        }
    }

    /// get additional_resources
    pub fn additional_resources(&self) -> &Vec<AdditionalResource> {
        match self {
            Self::Jig(jig) => &jig.jig_data.additional_resources,
            Self::Course(course) => &course.course_data.additional_resources,
            Self::Resource(resource) => &resource.resource_data.additional_resources,
        }
    }

    /// get translated_description
    pub fn translated_description(&self) -> &HashMap<String, String> {
        match self {
            Self::Jig(jig) => &jig.jig_data.translated_description,
            Self::Course(course) => &course.course_data.translated_description,
            Self::Resource(resource) => &resource.resource_data.translated_description,
        }
    }

    /// get theme
    pub fn theme(&self) -> ThemeId {
        match self {
            Self::Jig(jig) => jig.jig_data.theme,
            Self::Course(_) => ThemeId::default(),
            Self::Resource(_) => ThemeId::default(),
        }
    }

    /// get live_up_to_date
    pub fn live_up_to_date(&self) -> bool {
        match self {
            Self::Jig(jig) => jig.live_up_to_date,
            Self::Course(course) => course.live_up_to_date,
            Self::Resource(resource) => resource.live_up_to_date,
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
            UserOrMe::User(id) => serializer.collect_str(&id),
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

/// Sort browse results by timestamp
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Display)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum OrderBy {
    /// Order Asset results by timestamp created_at
    #[strum(serialize = "Created")]
    CreatedAt = 0,

    /// Order Asset results by timestamp published_at
    #[strum(serialize = "Published")]
    PublishedAt = 1,
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

impl FromStr for PrivacyLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "public" => Self::Public,
            "unlisted" => Self::Unlisted,
            "private" => Self::Private,
            _ => return Err(anyhow!("invalid")),
        })
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

impl Default for DraftOrLive {
    fn default() -> Self {
        Self::Live
    }
}

impl DraftOrLive {
    /// create draft variant
    pub fn draft() -> Self {
        Self::Draft
    }

    /// create live variant
    pub fn live() -> Self {
        Self::Live
    }

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

    /// get str `draft` of `live`
    pub fn as_str(&self) -> &'static str {
        match self {
            DraftOrLive::Draft => "draft",
            DraftOrLive::Live => "live",
        }
    }
}

impl FromStr for DraftOrLive {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Self::Draft),
            "live" => Ok(Self::Live),
            s => Err(format!("Can't create DraftFroLive from {:?}", s)),
        }
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
