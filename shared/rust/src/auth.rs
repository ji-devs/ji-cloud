use crate::user::UserRole;
use serde::{Deserialize, Serialize};

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
pub enum RegisterError {
    EmptyDisplayname,
    EmptyFirstname,
    EmptyLastname,
    TakenEmail,
    TakenId,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClaims {
    pub id: String,
    pub csrf: Option<String>,
    pub roles: Vec<UserRole>,
}
