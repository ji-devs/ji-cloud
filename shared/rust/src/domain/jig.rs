//! Types for JIGs.
pub mod curation;

pub mod report;
use macros::make_path_parts;
pub use report::{JigReport, ReportId};

pub mod player;
pub use player::{JigPlayerSettings, TextDirection};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};
use uuid::Uuid;

use super::{
    additional_resource::AdditionalResource,
    asset::{DraftOrLive, OrderBy, PrivacyLevel, UserOrMe},
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId},
    module::LiteModule,
    user::UserId,
};
use crate::{api::endpoints::PathPart, domain::module::body::ThemeId};

/// Wrapper type around [`Uuid`], represents the ID of a JIG.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, PathPart, Hash)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigId(pub Uuid);

make_path_parts!(JigCreatePath => "/v1/jig");

/// Request to create a new JIG.
///
/// This creates the draft and live [JigData](JigData) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigCreateRequest {
    /// The JIG's name.
    #[serde(default)]
    pub display_name: String,

    /// This JIG's age ranges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub age_ranges: Vec<AgeRangeId>,

    /// This jig's affiliations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub affiliations: Vec<AffiliationId>,

    /// The language the jig uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(default)]
    pub language: String,

    /// The jig's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,

    /// Description of the jig. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// Default player settings for this jig.
    #[serde(default)]
    pub default_player_settings: JigPlayerSettings,
}

/// The over-the-wire representation of a JIG's data. This can either be the live copy or the draft copy.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigData {
    /// When the JIG was first created.
    pub created_at: DateTime<Utc>,

    /// When the jig was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// Whether the JIG data is the live copy or the draft.
    pub draft_or_live: DraftOrLive,

    /// The JIG's name.
    pub display_name: String,

    /// The JIG's remaining modules.
    ///
    /// NOTE: the first module will always exist and will always be of type cover
    pub modules: Vec<LiteModule>,

    /// This jig's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// This jig's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The language the jig uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// The jig's categories.
    pub categories: Vec<CategoryId>,

    /// Additional resources of this JIG.
    pub additional_resources: Vec<AdditionalResource>,

    /// Description of the jig.
    pub description: String,

    /// Default player settings for this jig.
    pub default_player_settings: JigPlayerSettings,

    /// Theme for this jig, identified by `[ThemeId](module::body::ThemeId)`.
    pub theme: ThemeId,

    /// Background audio
    pub audio_background: Option<AudioBackground>,

    /// Audio effects
    pub audio_effects: AudioEffects,

    /// The privacy level on the JIG.
    pub privacy_level: PrivacyLevel,

    /// Lock this jig
    pub locked: bool,

    /// Other keywords used to searched for jigs
    pub other_keywords: String,

    /// translated keywords used to searched for jigs
    pub translated_keywords: String,

    /// translated descriptions
    #[serde(default)]
    pub translated_description: HashMap<String, String>,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigAdminData {
    /// Rating for jig, weighted for jig search
    #[serde(default)]
    pub rating: Option<JigRating>,

    /// if true does not appear in search
    pub blocked: bool,

    /// Indicates jig has been curated by admin
    pub curated: bool,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigUpdateAdminDataRequest {
    /// Rating for jig, weighted for jig search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<JigRating>,

    /// if true does not appear in search
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked: Option<bool>,

    /// Indicates jig has been curated by admin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curated: Option<bool>,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigAdminUpdateData {
    /// Rating for jig, weighted for jig search
    pub rating: Option<Option<JigRating>>,

    /// if true does not appear in search
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked: Option<bool>,

    /// Indicates jig has been curated by admin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curated: Option<bool>,
}

/// Admin rating for Jig
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum JigRating {
    #[allow(missing_docs)]
    One = 1,
    #[allow(missing_docs)]
    Two = 2,
    #[allow(missing_docs)]
    Three = 3,
}

impl TryFrom<u8> for JigRating {
    type Error = ();

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

/// Audio for background music
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum AudioBackground {
    #[allow(missing_docs)]
    FunForKids = 0,
    #[allow(missing_docs)]
    DancingHappy = 1,
    #[allow(missing_docs)]
    Jigzi1 = 2,
    #[allow(missing_docs)]
    Jigzi2 = 3,
    #[allow(missing_docs)]
    Jigzi3 = 4,
    #[allow(missing_docs)]
    Awestruck = 5,
    #[allow(missing_docs)]
    BayBounce = 6,
    #[allow(missing_docs)]
    CalmAndReflective = 7,
    #[allow(missing_docs)]
    DayWithoutRain = 8,
    #[allow(missing_docs)]
    DestinationFreedom = 9,
    #[allow(missing_docs)]
    FutureMemories = 10,
    #[allow(missing_docs)]
    HappyInstrumental = 11,
    #[allow(missing_docs)]
    HappyWhistle = 12,
    #[allow(missing_docs)]
    KidsInstrumental = 13,
    #[allow(missing_docs)]
    PartyKids = 14,
    #[allow(missing_docs)]
    RhythmKids = 15,
    #[allow(missing_docs)]
    SunKissed = 16,

    // legacy only background audio
    #[allow(missing_docs)]
    LegacyCuckooToYou = 101,
    #[allow(missing_docs)]
    LegacyFirstEtude = 102,
    #[allow(missing_docs)]
    LegacyHanerotHalalu = 103,
    #[allow(missing_docs)]
    LegacyIslandRomp = 104,
    #[allow(missing_docs)]
    LegacyJiTap = 105,
    #[allow(missing_docs)]
    LegacyMaozTzur = 106,
    #[allow(missing_docs)]
    LegacyModehAni = 107,
    #[allow(missing_docs)]
    LegacyMonkeyBars = 108,
    #[allow(missing_docs)]
    LegacyMorningZoo = 109,
    #[allow(missing_docs)]
    LegacyNapTime = 110,
    #[allow(missing_docs)]
    LegacyPlaylandMarch = 111,
    #[allow(missing_docs)]
    LegacyShehechiyanu = 112,
    #[allow(missing_docs)]
    LegacySunAndNoClouds = 113,
    #[allow(missing_docs)]
    LegacyTeddysBear = 114,
    #[allow(missing_docs)]
    LegacyWanderingWalrus = 115,
    #[allow(missing_docs)]
    LegacyWindupLullaby = 116,
}

impl AudioBackground {
    /// Get all enum variants (except legacy)
    pub fn variants() -> Vec<Self> {
        vec![
            Self::FunForKids,
            Self::DancingHappy,
            Self::Jigzi1,
            Self::Jigzi2,
            Self::Jigzi3,
            Self::Awestruck,
            Self::BayBounce,
            Self::CalmAndReflective,
            Self::DayWithoutRain,
            Self::DestinationFreedom,
            Self::FutureMemories,
            Self::HappyInstrumental,
            Self::HappyWhistle,
            Self::KidsInstrumental,
            Self::PartyKids,
            Self::RhythmKids,
            Self::SunKissed,
        ]
    }
}

/// Audio Effects
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioEffects {
    /// Positive audio feedback
    pub feedback_positive: HashSet<AudioFeedbackPositive>,

    /// Negative audio feedback
    pub feedback_negative: HashSet<AudioFeedbackNegative>,
}

/// Negative Audio Feedback
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum AudioFeedbackNegative {
    #[allow(missing_docs)]
    Bang = 0,
    #[allow(missing_docs)]
    Boing = 1,
    #[allow(missing_docs)]
    Buzz = 2,
    #[allow(missing_docs)]
    Buzzer = 3,
    #[allow(missing_docs)]
    Clang = 4,
    #[allow(missing_docs)]
    Clicks = 5,
    #[allow(missing_docs)]
    Incorrect = 6,
    #[allow(missing_docs)]
    JumpWrong = 7,
    #[allow(missing_docs)]
    NotRight = 8,
    #[allow(missing_docs)]
    OhNo = 9,
    #[allow(missing_docs)]
    ShortClang = 10,
    #[allow(missing_docs)]
    Whir = 11,
}
impl AudioFeedbackNegative {
    /// Get all enum variants
    pub fn variants() -> Vec<AudioFeedbackNegative> {
        vec![
            AudioFeedbackNegative::Bang,
            AudioFeedbackNegative::Boing,
            AudioFeedbackNegative::Buzz,
            AudioFeedbackNegative::Buzzer,
            AudioFeedbackNegative::Clang,
            AudioFeedbackNegative::Clicks,
            AudioFeedbackNegative::Incorrect,
            AudioFeedbackNegative::JumpWrong,
            AudioFeedbackNegative::NotRight,
            AudioFeedbackNegative::OhNo,
            AudioFeedbackNegative::ShortClang,
            AudioFeedbackNegative::Whir,
        ]
    }
}

/// Positive Audio Feedback
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum AudioFeedbackPositive {
    #[allow(missing_docs)]
    Correct = 0,
    #[allow(missing_docs)]
    Keys = 1,
    #[allow(missing_docs)]
    Magic = 2,
    #[allow(missing_docs)]
    Notes = 3,
    #[allow(missing_docs)]
    StarPing = 4,
    #[allow(missing_docs)]
    Ting = 5,
    #[allow(missing_docs)]
    Trumpet = 6,
    #[allow(missing_docs)]
    VoiceAwesome = 7,
    #[allow(missing_docs)]
    VoicesHurray = 8,
    #[allow(missing_docs)]
    VoiceYippee = 9,
    #[allow(missing_docs)]
    Xylophone = 10,
    #[allow(missing_docs)]
    Yes = 11,
}
impl AudioFeedbackPositive {
    /// Get all enum variants
    pub fn variants() -> Vec<AudioFeedbackPositive> {
        vec![
            AudioFeedbackPositive::Correct,
            AudioFeedbackPositive::Keys,
            AudioFeedbackPositive::Magic,
            AudioFeedbackPositive::Notes,
            AudioFeedbackPositive::StarPing,
            AudioFeedbackPositive::Ting,
            AudioFeedbackPositive::Trumpet,
            AudioFeedbackPositive::VoiceAwesome,
            AudioFeedbackPositive::VoicesHurray,
            AudioFeedbackPositive::VoiceYippee,
            AudioFeedbackPositive::Xylophone,
            AudioFeedbackPositive::Yes,
        ]
    }
}

/// The response returned when a request for `GET`ing a jig is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JigResponse {
    /// The ID of the JIG.
    pub id: JigId,

    /// When (if at all) the JIG has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,

    /// The ID of the JIG's original creator ([`None`] if unknown).
    pub creator_id: Option<UserId>,

    /// The current author
    pub author_id: Option<UserId>,

    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,

    /// Number of likes on Jig
    pub likes: i64,

    /// Number of plays Jig
    pub plays: i64,

    /// Live is current to Draft
    pub live_up_to_date: bool,

    /// The data of the requested JIG.
    pub jig_data: JigData,

    /// Liked by current user.
    pub is_liked: bool,

    /// Admin data for Jig
    pub admin_data: JigAdminData,
}

make_path_parts!(JigGetLivePath => "/v1/jig/{}/live" => JigId);

make_path_parts!(JigGetDraftPath => "/v1/jig/{}/draft" => JigId);

make_path_parts!(JigUpdateDraftDataPath => "/v1/jig/{}" => JigId);

/// Request for updating a JIG's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigUpdateDraftDataRequest {
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
    pub author_id: Option<UserId>,

    /// Description of the jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Default player settings for this jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_player_settings: Option<JigPlayerSettings>,

    /// Theme for this jig, identified by `[ThemeId](module::body::ThemeId)`.
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

    /// Privacy level for the jig.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_level: Option<PrivacyLevel>,

    /// Additional keywords for searches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other_keywords: Option<String>,
}

make_path_parts!(JigPublishPath => "/v1/jig/{}/draft/publish" => JigId);

make_path_parts!(JigBrowsePath => "/v1/jig/browse");

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

    /// Optionally browse by draft or live.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_or_live: Option<DraftOrLive>,

    /// Optionally filter jig by their privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// Optionally filter jig by blocked status
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,

    /// Optionally filter by `additional resources`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceTypeId>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
}

/// Response for [`Browse`](crate::api::endpoints::jig::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigBrowseResponse {
    /// the jigs returned.
    pub jigs: Vec<JigResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of jigs found
    pub total_jig_count: u64,
}

make_path_parts!(JigSearchPath => "/v1/jig");

/// Search for jigs via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the jigs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `language`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

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

    /// Optionally filter by `additional resources`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceTypeId>,

    /// Optionally filter by `categories`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,

    /// Optionally filter by `is_published`. This means that the jig's `publish_at < now()`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author's id
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// Optionally filter by the author's name
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,

    /// Optionally search for jigs using keywords
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_keywords: Option<String>,

    /// Optionally search for jigs using translated keyword
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_keywords: Option<String>,

    /// Optionally search for jigs by privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// Optionally search for blocked or non-blocked jigs
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct JigIdResponse {
    /// The ID of the jig
    pub id: JigId,
}

make_path_parts!(JigClonePath => "/v1/jig/{}/clone" => JigId);

make_path_parts!(JigDeletePath => "/v1/jig/{}" => JigId);

make_path_parts!(JigDeleteAllPath => "/v1/jig");

make_path_parts!(JigCoverPath => "/v1/jig/{}/cover" => JigId);

make_path_parts!(JigCountPath => "/v1/jig/count");

/// Response for total count of public and published jig.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCountResponse {
    /// Total number of public and published jigs.
    pub total_count: u64,
}

make_path_parts!(JigLikePath => "/v1/jig/{}/like" => JigId);

make_path_parts!(JigUnlikePath => "/v1/jig/{}/unlike" => JigId);

make_path_parts!(JigLikedPath => "/v1/jig/{}/like" => JigId);

/// Response for whether a user has liked a JIG.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JigLikedResponse {
    /// Whether the authenticated user has liked the current JIG
    pub is_liked: bool,
}

make_path_parts!(JigPlayPath => "/v1/jig/{}/play" => JigId);

make_path_parts!(JigAdminDataUpdatePath => "/v1/jig/{}/admin" => JigId);

into_uuid![JigId];
