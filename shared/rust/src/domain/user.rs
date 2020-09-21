//! Types for users.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

use super::meta::SubjectId;

/// Represents a user's permissions.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[non_exhaustive]
#[repr(i16)]
pub enum UserScope {
    /// The user has access to everything(?)
    Admin = 1,

    /// The user can create/delete/modify categories
    ManageCategory = 2,

    /// The user can create/delete/modify images.
    ManageImage = 3,
}

impl TryFrom<i16> for UserScope {
    type Error = ();

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Admin),
            2 => Ok(Self::ManageCategory),
            3 => Ok(Self::ManageImage),
            _ => Err(()),
        }
    }
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
    pub scopes: Vec<UserScope>,

    /// When the user was created.
    pub created_at: DateTime<Utc>,

    /// When the user was last updated.
    pub updated_at: Option<DateTime<Utc>>,

    /// The organization that the user belongs to.
    pub organization: String,

    /// The user's taught subjects.
    pub subjects: Vec<SubjectId>,
}
