//! Types for users.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

use crate::domain::{
    image::ImageId,
    meta::{AffiliationId, AgeRangeId, SubjectId},
};

/// Represents a user's permissions.
///
/// Note: 5 was "ManageModule", and has been deleted, but cannot be replaced(?)
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
            _ => anyhow::bail!("Scope {} is invalid", i),
        }
    }
}

/// Query to lookup a user by unique data
/// no filters will return that the user does not exist.
/// multiple filters will act as a logical `OR` of them (multiple choices will return an arbitrary user).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserLookupQuery {
    /// The user ID we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    /// The name we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Publicly accessible information about a user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtherUser {
    /// The user's id.
    pub id: Uuid,
}

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

/// A user's profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    /// The user's id.
    pub id: Uuid,

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

    /// The user's preferred language.
    pub language: String,

    /// The user's preferred locale.
    pub locale: String,

    /// Does the user want educational resources sent to them?
    pub opt_into_edu_resources: bool,

    /// Is the user over 18 years old?
    pub over_18: bool,

    /// The user's timezone.
    pub timezone: chrono_tz::Tz,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<String>,

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

impl UserProfile {
    /// Returns the display name for UI purposes
    pub fn display_name(&self) -> String {
        format!("{} {}", self.given_name, self.family_name)
            .trim()
            .to_string()
    }
}

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

    // todo: create a struct that enforces format like `en_us`
    /// the language the user prefers to communicate with.
    pub language: String,

    /// The locale that should be used for the user.
    pub locale: String,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<String>,

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

    // todo: create a struct that enforces format like `en_us`
    /// the language the user prefers to communicate with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// The locale that should be used for the user.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// the timezone that the user uses.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<chrono_tz::Tz>,

    /// Does the user want educational resources sent to them?
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_into_edu_resources: Option<bool>,

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
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<Option<String>>,

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

/// Request for [`Create`](crate::api::endpoints::user::Create)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// The new user's email
    pub email: String,

    /// The new user's password
    pub password: String,
}

/// Request for [`ResetPassword`](crate::api::endpoints::user::ResetPassword)
#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    /// The email to request a password reset for
    pub email: String,
}

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

/// Request for [`CreateColor`](crate::api::endpoints::user::CreateColor), [`UpdateColor`](crate::api::endpoints::user::UpdateColor)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserColorValueRequest {
    /// the color to add/change to.
    pub color: rgb::RGBA8,
}

/// Response for [`GetColors`](crate::api::endpoints::user::GetColors)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserColorResponse {
    /// The user's colors.
    pub colors: Vec<rgb::RGBA8>,
}

/// Request for [`CreateFont`](crate::api::endpoints::user::CreateFont), [`UpdateFont`](crate::api::endpoints::user::UpdateFont)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFontNameRequest {
    /// Name of the font to add/change.
    pub name: String,
}

/// Response for [`GetFonts`](crate::api::endpoints::user::GetFonts)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFontResponse {
    /// Names of the user's fonts.
    pub names: Vec<String>,
}
