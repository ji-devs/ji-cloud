//! Types for authorization.

use super::meta::{AffiliationId, AgeRangeId, SubjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The name to use for JWT cookies.
pub const JWT_COOKIE_NAME: &str = "X-JWT";

/// The name of the CSRF header.
pub const CSRF_HEADER_NAME: &str = "X-CSRF";

// todo: do we even need this?
#[derive(Serialize, Deserialize)]
/// Response for a successful signin.
pub struct SigninSuccess {
    /// The csrf of the signin.
    pub csrf: String,
}

#[derive(Serialize, Deserialize)]
/// Response for a successful SSO.
pub struct SingleSignOnSuccess {
    /// The jwt token to be sent as authorization for future requests.
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
/// Response for a successful registration.
pub enum RegisterSuccess {
    // fixme: what does this even do?
    #[allow(missing_docs)]
    Signin(String),
    // fixme: what does this even do?
    #[allow(missing_docs)]
    ConfirmEmail,
}

#[derive(Serialize, Deserialize)]
/// Request to be sent when registering a new user.
pub struct RegisterRequest {
    /// The user's username.
    ///
    /// This must be unique.
    pub username: String,

    /// The user's email.
    ///
    /// This must be unique.
    pub email: String,

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
    pub geocode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The claims that are used as part of the user's jwt.
pub struct AuthClaims {
    // fixme: use `sub` (short for subject, it's a standard jwt claim.)
    /// The user claimed by the jwt.
    pub id: Uuid,
    /// The csrf that must match for the jwt to be considered valid.
    pub csrf: Option<String>,
}
