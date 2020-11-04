use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct FirebaseError {
    pub code: String 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FirebaseUserInfo {
    Google(GoogleUserInfo),
    Email(EmailUserInfo)
}

impl FirebaseUserInfo {
    pub fn email_verified (&self) -> bool {
        match self {
            Self::Google(user) => user.email_verified,
            Self::Email(user) => user.email_verified,
        }
    }
    pub fn email(&self) -> &str {
        match self {
            Self::Google(user) => &user.email,
            Self::Email(user) => &user.email,
        }
    }
    pub fn token(&self) -> &str {
        match self {
            Self::Google(user) => &user.token,
            Self::Email(user) => &user.token,
        }
    }
    pub fn firebase_id(&self) -> &str {
        match self {
            Self::Google(user) => &user.firebase_id,
            Self::Email(user) => &user.firebase_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GoogleUserInfo {
    pub avatar: String,
    pub email: String,
    pub name: String,
    pub token: String,
    pub firebase_id: String,
    pub email_verified: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmailUserInfo {
    pub email: String,
    pub token: String,
    pub firebase_id: String,
    pub email_verified: bool,
}
