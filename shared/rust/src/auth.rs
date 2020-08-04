use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use actix_web::HttpResponse;
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
pub enum RegisterError {
    EmptyDisplayName,
    TakenEmail,
    TakenId,
    InternalServerError,
}

#[cfg(feature = "backend")]
impl From<RegisterError> for actix_web::Error {
    fn from(e: RegisterError) -> actix_web::Error {
        match e {
            RegisterError::InternalServerError => HttpResponse::InternalServerError().into(),
            e => HttpResponse::UnprocessableEntity().json(e).into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub display_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClaims {
    pub id: Uuid,
    pub csrf: Option<String>,
}
