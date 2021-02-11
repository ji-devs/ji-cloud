//! Types for authorization.

use super::meta::{AffiliationId, AgeRangeId, SubjectId};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The name to use for JWT cookies.
pub const AUTH_COOKIE_NAME: &str = "X-AUTH";

/// The name of the CSRF header.
pub const CSRF_HEADER_NAME: &str = "X-CSRF";

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
/// Request to be sent when registering a new user.
pub struct RegisterRequest {
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
