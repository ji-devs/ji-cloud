use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

// note that this is unstable and will totally be split into many things later on
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[repr(i16)]
pub enum UserScope {
    Admin = 1,
}

impl TryFrom<i16> for UserScope {
    type Error = ();

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Admin),
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoSuchUserError {}
