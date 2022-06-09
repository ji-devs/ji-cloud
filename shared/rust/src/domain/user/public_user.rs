//! Types for public users.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    additional_resource::AdditionalResource, asset::UserOrMe, badge::BadgeId, image::ImageId,
};

/// A lite profile for other Users to view
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    /// User Id
    pub id: Uuid,
    /// Username of User
    pub username: String,
    /// First name of User
    pub given_name: String,
    /// Lastname of User
    pub family_name: String,
    /// Bio of User
    pub bio: String,
    /// Profile image of User
    pub profile_image: Option<ImageId>,
    /// Language of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>, // only here if language_public is true
    /// Organization of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>, // only here if organization_public is true
    /// Persona of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub persona: Vec<String>, // only here if persona_public is true
    /// Location of User
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>, // only here if location_public is true
    /// Badges associated of User
    #[serde(default)]
    pub badges: Vec<BadgeId>,
}

/// Query for [`Browse`](crate::api::endpoints::user::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserBrowseQuery {
    /// The page number of the Courses to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// A lite profile for other Users to view
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResponse {
    /// User Id
    pub users: Vec<PublicUser>,
    /// Pages
    pub pages: u32,
    /// number of users
    pub total_user_count: u64,
}

/// Query for [`Browse`](crate::api::endpoints::user::Search).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchPublicUserQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Badges to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by user's id
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

    /// Optionally filter by the user's language
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optionally filter by the user's organization
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Optionally filter by the user's persona(s)
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<Vec<String>>,

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
    /// Pages
    pub pages: u32,
    /// number of users
    pub total_user_count: u64,
}

/// Query for [`Browse`](crate::api::endpoints::user::BrowseUserJigs).
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

/// Query for [`Browse`](crate::api::endpoints::user::BrowseUserResources).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResourcesQuery {
    /// The page number of the User Jigs to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Browse User's Additional Resources
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserResourcesResponse {
    /// the resources returned.
    pub resources: Vec<AdditionalResource>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of resources found
    pub total_resource_count: u64,
}

/// Query for [`Browse`](crate::api::endpoints::user::BrowseUserCourses).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserCoursesQuery {
    /// The page number of the User Courses to fetch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Query for [`Browse`](crate::api::endpoints::user::BrowseFollowers).
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
    /// the followers returned.
    pub followers: Vec<PublicUser>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of followers found
    pub total_follower_count: u64,
}

/// Query for [`Browse`](crate::api::endpoints::user::BrowseFollowing).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowsePublicUserFollowingsQuery {
    /// The page number of the User Followers to fetch.
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
    /// the followings returned.
    pub followings: Vec<PublicUser>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of followings found
    pub total_following_count: u64,
}