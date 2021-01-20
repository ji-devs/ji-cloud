use dominator_helpers::{futures::AsyncLoader};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use crate::firebase::*;
use wasm_bindgen::prelude::*;

pub struct State {
    pub loader: AsyncLoader,
    pub username: RefCell<String>,
    pub password: RefCell<String>,
    pub status: Mutable<Option<Status>>,
}
impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            username: RefCell::new("".to_string()),
            password: RefCell::new("".to_string()),
            status: Mutable::new(None),
        }
    }

    pub fn clear_status(&self) {
        self.status.set(None);
    }

    pub fn username_error(&self) -> impl Signal<Item = &'static str> {
        self.status
            .signal_cloned()
            .map(|err| {
                 err
                 .and_then(|err| err.username_error())
                 .unwrap_or("")
            })
    }

    pub fn password_error(&self) -> impl Signal<Item = &'static str> {
        self.status
            .signal_cloned()
            .map(|err| {
                 err
                 .and_then(|err| err.password_error())
                 .unwrap_or("")
            })
    }
}


#[derive(Debug, Clone)]
pub enum Status {
    NoSuchFirebaseUser,
    NoSuchDbUser(FirebaseUserInfo),
    BadPassword,
    UnknownFirebase,
    Technical,
    PasswordResetSent,
    InvalidEmail,
    ConfirmEmail
}

impl Status {
    pub fn username_error(&self) -> Option<&'static str> {
        match self {
            Self::NoSuchFirebaseUser => Some("no such user!"),
            Self::InvalidEmail => Some("invalid email"),
            Self::ConfirmEmail => Some("need to confirm your email!"),
            _ => None
        }
    }
    pub fn password_error(&self) -> Option<&'static str> {
        match self {
            Self::BadPassword => Some("wrong password!"),
            Self::PasswordResetSent => Some("password reset link sent!"),
            _ => None
        }
    }

    pub fn technical_error(&self) -> String {
        match self {
            Self::UnknownFirebase => "firebase error!",
            _ => "technical error!"
        }.to_string()
    }

    pub fn from_firebase_err(err:JsValue) -> Self {
        match serde_wasm_bindgen::from_value::<FirebaseError>(err) {
            Ok(err) => {
                let code:&str = err.code.as_ref();
                log::info!("{}", code);
                let status = match code {
                    "auth/wrong-password" => Self::BadPassword,
                    "auth/user-not-found" => Self::NoSuchFirebaseUser,
                    "auth/invalid-email" => Self::InvalidEmail,
                    "internal/confirm-email" => Self::ConfirmEmail,
                    _ => {
                        log::warn!("firebase error: {}", code);
                        Self::UnknownFirebase
                    }
                };
                status
            },
            Err(_) => {
                Self::Technical
            }

        }
    }
}
