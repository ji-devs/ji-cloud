use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const JWT_COOKIE_NAME: &'static str = "X-JWT";
pub const CSRF_HEADER_NAME: &str = "X-CSRF";

#[derive(Serialize, Deserialize)]
pub struct SigninSuccess {
    pub csrf: String,
}

#[derive(Serialize, Deserialize)]
pub struct SingleSignOnSuccess {
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub enum RegisterSuccess {
    Signin(String),
    ConfirmEmail,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub over_18: bool,
    pub given_name: String,
    pub family_name: String,
    // todo: create a struct that enforces format like `en_us`
    pub language: String,
    pub locale: String,
    pub timezone: chrono_tz::Tz,
    pub opt_into_edu_resources: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClaims {
    pub id: Uuid,
    pub csrf: Option<String>,
}
