use std::collections::HashSet;
use core::firebase::*;

pub type Token = String;
pub type Email = String;

#[derive(Clone, Debug)]
pub enum Step {
    Start,
    One,
    Two,
    Three,
    Final
}

#[derive(Clone, Debug, Default)]
pub struct RegisterData {
    pub token: Option<String>,
    pub email: Option<String>,
    pub user_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub edu_resources: bool,
    pub lang: Option<String>,
    pub location_json: Option<serde_json::Value>,
    pub affiliations: HashSet<String>,
    pub age_ranges: HashSet<String>,
    pub subjects: HashSet<String>,
    pub confirmed_email: bool,
}

impl From<Option<FirebaseUserInfo>> for RegisterData {
    fn from(user:Option<FirebaseUserInfo>) -> Self {
        let mut data = Self::default();

        //continue registration flow
        if let Some(user) = user {
            data.token = Some(user.token().to_string());
            data.email = Some(user.email().to_string());
            data.confirmed_email = user.email_verified();
        }

        data
    }
}

#[derive(Debug, Clone)]
pub enum RegisterStatus {
    Busy,
    Failure,
    ConfirmEmail,
    EmptyPw,
    PwMismatch,
    PwWeak,
    EmptyGivenName,
    EmptyLastName,
    EmptyUserName,
    EmptyEmail,
    EmailExists,
    UsernameExists,
    IdExists,
    Location,
    Language,
    Over18,
    UnknownFirebase,
    Technical 
}

#[derive(Clone, Copy, Debug)]
pub enum PwInvalid {
    Empty,
    Mismatch
}

impl RegisterStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Busy => "registering...",
            Self::Failure => "failed to register!",
            Self::ConfirmEmail => "confirm your email!",
            Self::EmptyPw => "supply a password!",
            Self::PwMismatch => "passwords don't match!",
            Self::PwWeak => "weak password!",
            Self::EmptyGivenName => "supply a first name!",
            Self::EmptyLastName => "supply a last name!",
            Self::EmptyUserName => "supply a user name!",
            Self::EmptyEmail => "supply an email address!",
            Self::Over18 => "Check the age restriction!",
            Self::Location => "Supply a location!",
            Self::Language => "Choose a language!",
            Self::UsernameExists => "Username in use!",
            Self::EmailExists => "Email in use!",
            Self::UnknownFirebase => "firebase error!",
            Self::IdExists => "id exists!",
            Self::Technical => "technical error!",
        }.to_string()
    }
}

