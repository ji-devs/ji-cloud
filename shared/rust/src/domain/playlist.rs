//! Types for Playlists.

use crate::domain::UpdateNonNullable;
use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    super::api::endpoints::PathPart,
    additional_resource::AdditionalResource,
    asset::{DraftOrLive, PrivacyLevel, UserOrMe},
    category::CategoryId,
    jig::JigId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId},
    module::LiteModule,
    user::UserId,
};

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a Playlist.
    pub struct PlaylistId
}

make_path_parts!(PlaylistCreatePath => "/v1/playlist");

/// Request to create a new Playlist.
///
/// This creates the draft and live [Playlist Data](Playlist Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCreateRequest {
    /// The Playlist's name.
    #[serde(default)]
    pub display_name: String,

    /// Description of the Playlist. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// This Playlist's age ranges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub age_ranges: Vec<AgeRangeId>,

    /// This Playlist's affiliations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub affiliations: Vec<AffiliationId>,

    /// The language the Playlist uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(default)]
    pub language: String,

    /// The Playlist's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,
}

/// The over-the-wire representation of a Playlist's data. This can either be the live copy or the draft copy.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistData {
    /// Whether the Playlist data is the live copy or the draft.
    pub draft_or_live: DraftOrLive,

    /// The Playlist's name.
    pub display_name: String,

    /// The language the Playlist uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// Description of the Playlist.
    pub description: String,

    /// When the Playlist was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// The privacy level on the Playlist.
    pub privacy_level: PrivacyLevel,

    /// Other keywords used to searched for Playlists
    pub other_keywords: String,

    /// translated keywords used to searched for Playlists
    pub translated_keywords: String,

    /// translated descriptions
    #[serde(default)]
    pub translated_description: HashMap<String, String>,

    /// This Playlist's cover.
    pub cover: Option<LiteModule>,

    /// This Playlist's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// This Playlist's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The Playlist's categories.
    pub categories: Vec<CategoryId>,

    /// Additional resources of this Playlist.
    pub additional_resources: Vec<AdditionalResource>,

    /// List of Jig Ids within the Playlist
    pub items: Vec<JigId>,
}

/// Admin rating for a course
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum PlaylistRating {
    #[allow(missing_docs)]
    One = 1,
    #[allow(missing_docs)]
    Two = 2,
    #[allow(missing_docs)]
    Three = 3,
}

impl TryFrom<u8> for PlaylistRating {
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

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAdminData {
    /// Rating for jig, weighted for jig search
    #[serde(default)]
    pub rating: Option<PlaylistRating>,

    /// if true does not appear in search
    pub blocked: bool,

    /// Indicates jig has been curated by admin
    pub curated: bool,

    /// Whether the resource is a premium resource
    pub premium: bool,
}

/// The response returned when a request for `GET`ing a Playlist is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
    /// The ID of the Playlist.
    pub id: PlaylistId,

    /// When (if at all) the Playlist has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,

    /// The ID of the Playlist's original creator ([`None`] if unknown).
    pub creator_id: Option<UserId>,

    /// The current author
    pub author_id: Option<UserId>,

    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,

    /// Number of likes on Playlist
    pub likes: i64,

    /// Number of plays Playlist
    pub plays: i64,

    /// Live is current to Draft
    pub live_up_to_date: bool,

    /// Liked by current user.
    pub is_liked: bool,

    /// The data of the requested Playlist.
    pub playlist_data: PlaylistData,

    /// Admin data for a course
    pub admin_data: PlaylistAdminData,
}

make_path_parts!(PlaylistGetLivePath => "/v1/playlist/{}/live" => PlaylistId);

make_path_parts!(PlaylistGetDraftPath => "/v1/playlist/{}/draft" => PlaylistId);

make_path_parts!(PlaylistUpdateDraftDataPath => "/v1/playlist/{}" => PlaylistId);

/// Request for updating a Playlist's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistUpdateDraftDataRequest {
    /// The Playlist's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<UserId>,

    /// Description of the Playlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// The language the Playlist uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,

    /// Privacy level for the Playlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_level: Option<PrivacyLevel>,

    /// Additional keywords for searches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other_keywords: Option<String>,

    /// The Playlist's categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// The Playlist's age ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// The Playlist's affiliations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// The Playlist's JIGs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub items: Option<Vec<JigId>>,
}

make_path_parts!(PlaylistPublishPath => "/v1/playlist/{}/draft/publish" => PlaylistId);

make_path_parts!(PlaylistBrowsePath => "/v1/playlist/browse");

/// Query for [`Browse`](crate::api::endpoints::playlist::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the Playlists to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally browse by draft or live.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_or_live: Option<DraftOrLive>,

    /// Optionally filter Playlist by their privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

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
}

/// Response for [`Browse`](crate::api::endpoints::playlist::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistBrowseResponse {
    /// the Playlists returned.
    pub playlists: Vec<PlaylistResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Playlists found
    pub total_playlist_count: u64,
}

make_path_parts!(PlaylistSearchPath => "/v1/playlist");

/// Search for Playlists via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Playlists to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `language`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optionally filter by `is_published`. This means that the Playlist's `publish_at < now()`.
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

    /// Optionally search for Playlists using keywords
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_keywords: Option<String>,

    /// Optionally search for Playlists using translated keyword
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_keywords: Option<String>,

    /// Optionally search for Playlists by privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,

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

    /// Optionally filter by `items`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<JigId>,

    /// Optionally filter playlists based off of existence of rating
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rated: Option<bool>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSearchResponse {
    /// the Playlists returned.
    pub playlists: Vec<PlaylistResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Playlists found
    pub total_playlist_count: u64,
}

/// Response for whether a user has liked a Playlist.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlaylistLikedResponse {
    /// Whether the authenticated user has liked the current Playlist
    pub is_liked: bool,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistUpdateAdminDataRequest {
    /// Rating for jig, weighted for jig search
    #[serde(default, skip_serializing_if = "UpdateNonNullable::is_keep")]
    pub rating: UpdateNonNullable<PlaylistRating>,

    /// if true does not appear in search
    #[serde(default, skip_serializing_if = "UpdateNonNullable::is_keep")]
    pub blocked: UpdateNonNullable<bool>,

    /// Indicates jig has been curated by admin
    #[serde(default, skip_serializing_if = "UpdateNonNullable::is_keep")]
    pub curated: UpdateNonNullable<bool>,

    /// Indicates jig is premium content
    #[serde(default, skip_serializing_if = "UpdateNonNullable::is_keep")]
    pub premium: UpdateNonNullable<bool>,
}

make_path_parts!(PlaylistDeletePath => "/v1/playlist/{}" => PlaylistId);

make_path_parts!(PlaylistClonePath => "/v1/playlist/{}/clone" => PlaylistId);

make_path_parts!(PlaylistLikePath => "/v1/playlist/{}/like" => PlaylistId);

make_path_parts!(PlaylistUnlikePath => "/v1/playlist/{}/unlike" => PlaylistId);

make_path_parts!(PlaylistLikedPath => "/v1/playlist/{}/like" => PlaylistId);

make_path_parts!(ListLikedPath => "/v1/playlist/likes");

/// Response for request for list of liked playlists.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListLikedRequest {
    /// The page number of the playlists to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}
/// Response for request for list of liked playlists.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListLikedResponse {
    /// the playlists returned.
    pub playlists: Vec<PlaylistResponse>,

    /// The total number of playlists liked
    pub total_playlist_count: u64,
}

make_path_parts!(PlaylistViewPath => "/v1/playlist/{}/view" => PlaylistId);

make_path_parts!(PlaylistAdminDataUpdatePath => "/v1/playlist/{}/admin" => PlaylistId);

/// A playlists export representation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminPlaylistExport {
    /// playlist ID
    pub id: PlaylistId,
    /// Description of the playlist.
    pub description: String,
    /// The playlist's name.
    pub display_name: String,
    /// Whether the resource is a premium resource
    pub premium: bool,
    /// if true does not appear in search
    pub blocked: bool,
    /// The current author
    pub author_id: Option<UserId>,
    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,
    /// Number of likes on playlist
    pub likes: i64,
    /// Number of plays playlist
    pub plays: i64,
    /// Rating for playlist, weighted for playlist search
    pub rating: Option<PlaylistRating>,
    /// The privacy level on the playlist.
    pub privacy_level: PrivacyLevel,
    /// When the playlist was first created.
    pub created_at: DateTime<Utc>,
    /// When (if at all) the playlist has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,
    /// The language the playlist uses.
    pub language: String,
}
