use serde::{Deserialize, Serialize};
use crate::user::UserRole;

pub const JWT_COOKIE_NAME:&'static str = "X-JWT";
pub const CSRF_HEADER_NAME:&'static str = "X-CSRF";

#[derive(Serialize, Deserialize)]
pub struct SigninSuccess {
    pub csrf: String
}

#[derive(Serialize, Deserialize)]
pub struct SigninEphemeralSuccess {
    pub jwt: String
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