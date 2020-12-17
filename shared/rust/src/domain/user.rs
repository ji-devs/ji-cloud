//! Types for users.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;

use super::meta::{AffiliationId, AgeRangeId, SubjectId};

/// Represents a user's permissions.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[non_exhaustive]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum UserScope {
    /// The user has access to everything(?)
    Admin = 1,

    /// The user can create/delete/modify categories
    ManageCategory = 2,

    /// The user can create/delete/modify images.
    ManageImage = 3,

    /// The user can create/delete/modify jigs.
    ManageJig = 4,

    /// The user can create/delete/modify module.
    ManageModule = 5,
}

impl TryFrom<i16> for UserScope {
    type Error = anyhow::Error;

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Admin),
            2 => Ok(Self::ManageCategory),
            3 => Ok(Self::ManageImage),
            4 => Ok(Self::ManageJig),
            5 => Ok(Self::ManageModule),
            _ => anyhow::bail!("Scope {} is invalid"),
        }
    }
}

/// Query to lookup a user by unique data
/// no filters will return that the user does not exist.
/// multiple filters will act as a logical `OR` of them (multiple choices will return an arbitrary user).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserLookupQuery {
    /// The user ID we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    /// The *Firebase* ID we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firebase_id: Option<String>,

    /// The name we're filtering by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Publically accessable information about a user.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct OtherUser {
    /// The user's id.
    pub id: Uuid,
}

/// A user's profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
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
