//! Types for users.

use chrono::{DateTime, NaiveDate, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize, Serializer};
use std::convert::TryFrom;

use crate::domain::billing::UserAccountSummary;
use crate::{
    api::endpoints::PathPart,
    domain::{
        circle::CircleId,
        image::ImageId,
        meta::{AffiliationId, AgeRangeId, SubjectId},
    },
};

pub mod public_user;

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a User.
    pub struct UserId
}

/// Represents a user's permissions.
///
/// Note: 5 was `ManageModule`, and has been deleted, but cannot be replaced(?)
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
#[non_exhaustive]
#[repr(i16)]
pub enum UserScope {
    /// The user has access to everything, implies all other scopes.
    Admin = 1,

    /// The user can create/delete/modify categories
    ManageCategory = 2,

    /// The user can create/delete/modify images.
    ManageImage = 3,

    /// The user can delete/modify *any* jigs.
    AdminJig = 4,

    /// The user can create/delete/modify animations.
    ManageAnimation = 6,

    /// The user can create/delete/modify locale entries.
    ManageEntry = 7,

    /// The user can create/modify/delete jigs of their own.
    ManageSelfJig = 8,

    /// The User can create/delete/modify audio files of their own.
    ManageAudio = 9,

    /// The User can create resource focused jigs.
    Resources = 10,
}

impl TryFrom<i16> for UserScope {
    type Error = anyhow::Error;

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Admin),
            2 => Ok(Self::ManageCategory),
            3 => Ok(Self::ManageImage),
            4 => Ok(Self::AdminJig),
            6 => Ok(Self::ManageAnimation),
            7 => Ok(Self::ManageEntry),
            8 => Ok(Self::ManageSelfJig),
            9 => Ok(Self::ManageAudio),
            10 => Ok(Self::Resources),
            _ => anyhow::bail!("Scope {} is invalid", i),
        }
    }
}

make_path_parts!(UserLookupPath => "/v1/user/lookup");

/// Query to lookup a user by unique data
/// no filters will return that the user does not exist.
/// multiple filters will act as a logical `OR` of them (multiple choices will return an arbitrary user).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserLookupQuery {
    /// The user ID we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UserId>,

    /// The name we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Publicly accessible information about a user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtherUser {
    /// The user's id.
    pub id: UserId,
}

make_path_parts!(ResetEmailPath => "/v1/user/me/reset-email");

/// Update user email request
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResetEmailRequest {
    /// user's email
    pub email: String,
}

/// Update user email response (returns the paseto token for the user)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResetEmailResponse {
    /// paseto token with user's email
    pub paseto_token: String,
}

/// user badge
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum UserBadge {
    /// Master teacher
    MasterTeacher = 0,
    /// JI team member
    JiTeam = 1,
}

impl UserBadge {
    /// get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            UserBadge::MasterTeacher => "Master Teacher",
            UserBadge::JiTeam => "JI Team",
        }
    }
}

/// A user's profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    /// The user's id.
    pub id: UserId,

    /// The user's username.
    pub username: String,

    /// The user's email address.
    pub email: String,

    /// Indicator for Oauth email
    pub is_oauth: bool,

    /// The user's given name (first name)
    pub given_name: String,

    /// The user's family name (last name)
    pub family_name: String,

    /// ID to the user's profile image in the user image library.
    pub profile_image: Option<ImageId>,

    /// The user's preferred application language.
    pub language_app: String,

    /// The user's preferred email language.
    pub language_emails: String,

    /// The user's preferred language.
    pub languages_spoken: Vec<String>,

    /// Does the user want educational resources sent to them?
    pub opt_into_edu_resources: bool,

    /// Is the user over 18 years old?
    pub over_18: bool,

    /// The user's timezone.
    pub timezone: chrono_tz::Tz,

    /// Bio for User
    pub bio: String,

    /// Badge of User
    pub badge: Option<UserBadge>,

    /// Allow location to be public
    #[serde(default)]
    pub location_public: bool,

    /// Allow organization to be public
    #[serde(default)]
    pub organization_public: bool, // default to false

    /// Allow persona to be public
    #[serde(default)]
    pub persona_public: bool, // default to false

    /// Allow languages_spoken to be public
    #[serde(default)]
    pub languages_spoken_public: bool, // default to false

    /// Allow bio to be public
    #[serde(default)]
    pub bio_public: bool, // default to false

    /// User associated Circles
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub circles: Vec<CircleId>,

    /// The scopes associated with the user.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<UserScope>,

    /// When the user was created.
    pub created_at: DateTime<Utc>,

    /// When the user was last updated.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    /// The organization that the user belongs to.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// The persona of the user
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub persona: Vec<String>,

    /// The user's taught subjects.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectId>,

    /// The user's age-ranges.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    /// The user's affiliations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    /// The user's location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,

    /// The user's account summary, if available.
    ///
    /// Note: This is not set when fetching a user profile. It must be explicitly set using a
    /// function such as [`db::account::get_user_account_summary()`]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_summary: Option<UserAccountSummary>,
}

/// User Response (used for Admin).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// The user's id.
    pub id: UserId,

    /// The user's username.
    pub username: String,

    /// The user's given name (first name)
    pub given_name: String,

    /// The user's family name (last name)
    pub family_name: String,

    /// The user's email address.
    pub email: String,

    /// The user's country.
    #[serde(default)]
    pub country: Option<String>,

    /// The user's state.
    #[serde(default)]
    pub state: Option<String>,

    /// The user's city.
    #[serde(default)]
    pub city: Option<String>,

    /// The user's associated organization/school.
    #[serde(default)]
    pub organization: Option<String>,

    /// The date the user signed up on .
    pub created_at: NaiveDate,

    /// The user's preferred email language for newsletters.
    pub language: String,

    /// The user's city.
    #[serde(default)]
    pub badge: Option<UserBadge>,
}

/// A user's profile export representation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileExport {
    /// The user's id.
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's email address.
    pub email: String,
    /// The user's given name (first name)
    pub given_name: String,
    /// The user's family name (last name)
    pub family_name: String,
    /// ID to the user's profile image in the user image library.
    pub profile_image: Option<ImageId>,
    /// The user's preferred application language.
    pub language_app: String,
    /// The user's preferred email language.
    pub language_emails: String,
    /// The user's preferred language.
    #[serde(default)]
    #[serde(serialize_with = "serialize_list")]
    pub languages_spoken: Vec<String>,
    /// When the user was created.
    pub created_at: DateTime<Utc>,
    /// When the user was last updated.
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    /// The organization that the user belongs to.
    #[serde(default)]
    pub organization: Option<String>,
    /// The persona of the user
    #[serde(default)]
    #[serde(serialize_with = "serialize_list")]
    pub persona: Vec<String>,
    /// The user's taught subjects.
    #[serde(default)]
    #[serde(serialize_with = "serialize_list")]
    pub subjects: Vec<String>,
    /// The user's age-ranges.
    #[serde(default)]
    #[serde(serialize_with = "serialize_list")]
    pub age_ranges: Vec<String>,
    /// The user's affiliations.
    #[serde(default)]
    #[serde(serialize_with = "serialize_list")]
    pub affiliations: Vec<String>,
    /// The user's city
    #[serde(default)]
    pub city: Option<String>,
    /// The user's country
    #[serde(default)]
    pub country: Option<String>,
    /// Whether this user has opted in to receive educational resources
    pub opt_into_edu_resources: bool,
}

fn serialize_list<S, T>(list: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    list.iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect::<Vec<String>>()
        .join(", ")
        .serialize(serializer)
}

impl UserProfile {
    /// Returns the display name for UI purposes
    pub fn display_name(&self) -> String {
        format!("{} {}", self.given_name, self.family_name)
            .trim()
            .to_string()
    }
}

make_path_parts!(VerifyEmailPath => "/v1/user/verify-email");

/// Request for [`VerifyEmail`](crate::api::endpoints::user::VerifyEmail)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VerifyEmailRequest {
    /// Attempt to verify the email
    Verify {
        /// The token to verify.
        token: String,
    },

    /// Resend a confirmation link if a verification is in progress
    Resend {
        /// The email to send a verification link to.
        email: String,
    },
}

make_path_parts!(VerifyResetEmailPath => "/v1/user/verify-reset-email");

/// Request for [`VerifyUpdateEmail`](crate::api::endpoints::user::VerifyEmail)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VerifyResetEmailRequest {
    /// Attempt to verify the email
    #[serde(rename_all = "camelCase")]
    Verify {
        /// paseto token
        paseto_token: String,

        /// Forcibly logout of all sessions.
        force_logout: bool,
    },

    /// Resend a confirmation link if a verification is in progress
    #[serde(rename_all = "camelCase")]
    Resend {
        /// paseto token
        paseto_token: String,
    },
}

make_path_parts!(CreateProfilePath => "/v1/user/me/profile");

/// Request for [`user::profile::Create`](crate::api::endpoints::user::CreateProfile)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    /// The user's username.
    ///
    /// This must be unique.
    pub username: String,

    /// Is the user >= 18 yeas old?
    pub over_18: bool,

    /// The user's given name / "first name".
    pub given_name: String,

    /// The user's family name / "last name".
    pub family_name: String,

    /// URL to the user's profile image. The API server uploads and processes the image so that the
    /// profile image is stored in Cloud Storage in the user image library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,

    /// The user's preferred application language.
    pub language_app: String,

    /// The user's preferred email language.
    pub language_emails: String,

    /// The user's preferred language.
    pub languages_spoken: Vec<String>,

    /// the timezone that the user uses.
    pub timezone: chrono_tz::Tz,

    // todo: does this have something to do with emails?
    /// Does the user want educational resources sent to them?
    pub opt_into_edu_resources: bool,

    /// The organization that the user belongs to.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// The persona of the user
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub persona: Vec<String>,

    /// The user's taught subjects.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<SubjectId>,

    /// The user's age-ranges.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    /// The user's affiliations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    /// The user's location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}

make_path_parts!(GetProfilePath => "/v1/user/me/profile");

make_path_parts!(PatchProfilePath => "/v1/user/me/profile");

/// Request for [`PatchProfile`](crate::api::endpoints::user::PatchProfile)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatchProfileRequest {
    /// The user's username.
    ///
    /// This must be unique.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// The user's given name / "first name".
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,

    /// The user's family name / "last name".
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,

    /// ID to the user's profile image in the user image library.
    #[serde(default)]
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<Option<ImageId>>,

    /// The user's bio
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// the language the user prefers the application to be in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_app: Option<String>,

    /// the language the user prefers emails to be in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_emails: Option<String>,

    /// the languages the user prefers.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_spoken: Option<Vec<String>>,

    /// the timezone that the user uses.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<chrono_tz::Tz>,

    /// Does the user want educational resources sent to them?
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_into_edu_resources: Option<bool>,

    /// Publicize Users organization
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_public: Option<bool>,

    /// Publicize user persona
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_public: Option<bool>,

    /// Publicize user lanuage
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_spoken_public: Option<bool>,

    /// Publicize user location
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_public: Option<bool>,

    /// Publicize user bio
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio_public: Option<bool>,

    /// The organization that the user belongs to.
    ///
    /// Field is updated if `Some(_)` with the inner contents.
    #[serde(default)]
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Option<String>>,

    /// The persona of the user.
    ///
    /// Field is updated if `Some(_)` with the inner contents.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<Vec<String>>,

    /// The user's taught subjects.
    ///
    /// If `Some`, replace the existing `SubjectId`s with this.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<SubjectId>>,

    /// The user's age-ranges.
    ///
    /// If `Some`, replace the existing `AgeRangeId`s with this.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// The user's affiliations.
    ///
    /// If `Some`, replace the existing `AffiliationId`s with this.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// The user's location.
    /// * If the outer `Option` is `None`, then no update is done,
    /// * If `Some(None)`, sets the location to `None`,
    /// * If `Some(Some(_))`, updates the user location to `Some(_)`.
    #[serde(default)]
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<serde_json::Value>>,
}

make_path_parts!(PatchProfileAdminDataPath => "/v1/user/me/profile/{}/admin-data" => UserId);

/// Request for [`PatchProfileAdminData`](crate::api::endpoints::user::PatchProfileAdminData)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatchProfileAdminDataRequest {
    /// Users badge
    #[serde(default)]
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<Option<UserBadge>>,
}

make_path_parts!(CreateUserPath => "/v1/user");

/// Request for [`Create`](crate::api::endpoints::user::Create)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// The new user's email
    pub email: String,

    /// The new user's password
    pub password: String,
}

make_path_parts!(ResetPasswordPath => "/v1/user/password-reset");

/// Request for [`ResetPassword`](crate::api::endpoints::user::ResetPassword)
#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    /// The email to request a password reset for
    pub email: String,
}

make_path_parts!(ChangePasswordPath => "/v1/user/me/password");

/// Request for [`ChangePassword`](crate::api::endpoints::user::ChangePassword)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangePasswordRequest {
    /// Change the email
    Change {
        /// The token to verify with
        token: String,

        /// The new password
        password: String,

        /// Forcibly logout of all sessions.
        force_logout: bool,
    },
}

make_path_parts!(UserDeletePath => "/v1/user/me");

// Colors

make_path_parts!(UserColorCreatePath => "/v1/user/me/color");

// i32 is color index
make_path_parts!(UserColorUpdatePath => "/v1/user/me/color/{}" => i32);

/// Request for [`CreateColor`](crate::api::endpoints::user::CreateColor), [`UpdateColor`](crate::api::endpoints::user::UpdateColor)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserColorValueRequest {
    /// the color to add/change to.
    pub color: rgb::RGBA8,
}

make_path_parts!(UserColorGetPath => "/v1/user/me/color");

/// Response for [`GetColors`](crate::api::endpoints::user::GetColors)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserColorResponse {
    /// The user's colors.
    pub colors: Vec<rgb::RGBA8>,
}

// i32 is color index
make_path_parts!(UserColorDeletePath => "/v1/user/me/color/{}" => i32);

// Fonts

make_path_parts!(UserFontCreatePath => "/v1/user/me/font");

// i32 is font index
make_path_parts!(UserFontUpdatePath => "/v1/user/me/font/{}" => i32);

/// Request for [`CreateFont`](crate::api::endpoints::user::CreateFont), [`UpdateFont`](crate::api::endpoints::user::UpdateFont)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFontNameRequest {
    /// Name of the font to add/change.
    pub name: String,
}

make_path_parts!(UserFontGetPath => "/v1/user/me/font");

/// Response for [`GetFonts`](crate::api::endpoints::user::GetFonts)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFontResponse {
    /// Names of the user's fonts.
    pub names: Vec<String>,
}

// i32 is font index
make_path_parts!(UserFontDeletePath => "/v1/user/me/font/{}" => i32);

//
// Browse users
//
// Authorization:
//  - Admin

/// Query for [`Browse`](crate::api::endpoints::user::Browse).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserBrowseQuery {
    /// filter User by Id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// The page number of the Users to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for [`Browse`](crate::api::endpoints::user::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBrowseResponse {
    /// the users returned.
    pub users: Vec<UserResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of users found
    pub total_user_count: u64,
}

make_path_parts!(UserBrowsePath => "/v1/user/browse");

//
// Search users
//
// Authorization:
//  - Admin

/// Query for [`Search`](crate::api::endpoints::user::Search).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// The page number of the Users to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for [`Search`](crate::api::endpoints::user::Search).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResponse {
    /// the users returned.
    pub users: Vec<UserResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of users found
    pub total_user_count: u64,
}

make_path_parts!(UserSearchPath => "/v1/user");
