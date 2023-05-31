//! Types for public users.

use crate::{
    api::endpoints::PathPart,
    domain::{
        additional_resource::AdditionalResource, asset::UserOrMe, circle::CircleId,
        csv_encode_uuids, from_csv, image::ImageId, user::UserId,
    },
};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

make_path_parts!(PublicUserGetPath => "/v1/user/{}/public" => UserId);

/// A lite profile for other Users to view
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    /// User Id
    pub id: UserId,

    /// Username of User
    pub username: String,

    /// First name of User
    pub given_name: String,

    /// Lastname of User
    pub family_name: String,

    /// Profile image of User
    pub profile_image: Option<ImageId>,

    /// Bio of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>, // only here if bio_public is true

    /// Language spoken of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_spoken: Option<Vec<String>>, // only here if languages_spoken_public is true

    /// Organization of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>, // only here if organization_public is true

    /// Persona of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<Vec<String>>, // only here if persona_public is true

    /// Location of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>, // only here if location_public is true

    /// Circles associated with User
    #[serde(default)]
    pub circles: Vec<CircleId>,
}

make_path_parts!(PublicUserBrowsePath => "/v1/user/public/browse");

/// Query for [`Browse`](crate::api::endpoints::user::public_user::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserBrowseQuery {
    /// The page number
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    pub page_limit: Option<u32>,

    /// Circle's that has user joined
    #[serde(default)]
    #[serde(serialize_with = "csv_encode_uuids")]
    #[serde(deserialize_with = "from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub circles: Vec<CircleId>,
}

/// A lite profile for other Users to view
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResponse {
    /// User Id
    pub users: Vec<PublicUser>,
    /// Pages
    pub pages: u32,
    /// Number of users with profiles
    pub total_user_count: u64,
}

make_path_parts!(PublicUserSearchPath => "/v1/user/public");

/// Query for [`Browse`](crate::api::endpoints::user::Search).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchPublicUserQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Circles to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by User's Id
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserOrMe>,

    /// Optionally filter by the user's username
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Optionally filter by the user's name
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optionally filter by the user's spoken language(s)
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_spoken: Option<Vec<String>>,

    /// Optionally filter by the user's organization
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Optionally filter by the user's persona(s)
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<Vec<String>>,

    /// Optionally filter by the user's bio
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// A lite profile for other Users to view
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchPublicUserResponse {
    /// User Id
    pub users: Vec<PublicUser>,
    /// Number of pages
    pub pages: u32,
    /// Number of User profiles
    pub total_user_count: u64,
}

make_path_parts!(BrowsePublicUserJigsPath => "/v1/user/{}/public/jig/browse" => UserId);

/// Query for [`Browse`](crate::api::endpoints::user::public_user::BrowseUserJigs).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserJigsQuery {
    /// The page number of the User Jigs to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

make_path_parts!(BrowsePublicUserResourcesPath => "/v1/user/{}/public/resource/browse" => UserId);

/// Query for [`Browse`](crate::api::endpoints::user::public_user::BrowseUserResources).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResourcesQuery {
    /// The page number of the User Resources to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for Browsing a User's Additional Resources
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResourcesResponse {
    /// The Additional Resources returned
    pub resources: Vec<AdditionalResource>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of additional resources belonging to user
    pub total_resource_count: u64,
}

make_path_parts!(BrowsePublicUserPlaylistsPath => "/v1/user/{}/public/playlist/browse" => UserId);

/// Query for [`Browse`](crate::api::endpoints::user::public_user::BrowseUserPlaylists).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserPlaylistsQuery {
    /// The page number of the User Playlists to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

make_path_parts!(BrowsePublicUserFollowersPath => "/v1/user/{}/public/follower/browse" => UserId);

/// Query for [`Browse`](crate::api::endpoints::user::public_user::BrowseFollowers).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserFollowersQuery {
    /// The page number of the User Followers to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Browse User's followers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserFollowersResponse {
    /// The follower profiles returned
    pub followers: Vec<PublicUser>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of followers found
    pub total_follower_count: u64,
}

make_path_parts!(BrowsePublicUserFollowingPath => "/v1/user/{}/public/following/browse" => UserId);

/// Query for [`Browse`](crate::api::endpoints::user::public_user::BrowseFollowing).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserFollowingsQuery {
    /// The page number of the Public User Followers to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Browse User's followings
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserFollowingResponse {
    /// The Public User Profiles of followings
    pub followings: Vec<PublicUser>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of followings found
    pub total_following_count: u64,
}

make_path_parts!(PublicUserFollowPath => "/v1/user/{}/follow" => UserId);

make_path_parts!(PublicUserUnfollowPath => "/v1/user/{}/unfollow" => UserId);
