use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

use super::meta::SubjectId;

// note that this is unstable and will totally be split into many things later on
#[derive(Deserialize, Serialize,  PartialEq, Debug, Clone)]
#[non_exhaustive]
#[repr(i16)]
pub enum UserScope {
    ManageCategory = 2,
    ManageImage = 3,
}

impl TryFrom<i16> for UserScope {
    type Error = ();

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            2 => Ok(Self::ManageCategory),
            3 => Ok(Self::ManageImage),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub language: String,
    pub locale: String,
    pub opt_into_edu_resources: bool,
    pub over_18: bool,
    pub timezone: chrono_tz::Tz,
    pub scopes: Vec<UserScope>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub organization: String,
    pub subjects: Vec<SubjectId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoSuchUserError {}
