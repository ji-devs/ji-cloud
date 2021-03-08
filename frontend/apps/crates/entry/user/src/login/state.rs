use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub struct State {
    pub loader: AsyncLoader,
    pub email: RefCell<String>,
    pub password: RefCell<String>,
    pub status: Mutable<Option<Status>>,
}
impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            email: RefCell::new("".to_string()),
            password: RefCell::new("".to_string()),
            status: Mutable::new(None),
        }
    }

    pub fn clear_email_status(&self) {
        if self.status.get_cloned().and_then(|x| x.email_error()).is_some() {
            self.status.set(None);
        }
    }
    pub fn clear_password_status(&self) {
        if self.status.get_cloned().and_then(|x| x.password_error()).is_some() {
            self.status.set(None);
        }
    }

    pub fn email_error(&self) -> impl Signal<Item = &'static str> {
        self.status
            .signal_cloned()
            .map(|err| {
                 err
                 .and_then(|err| err.email_error())
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
    BadPassword,
    Technical,
    PasswordResetSent,
    InvalidEmail,
    ConfirmEmail
}

impl Status {
    pub fn email_error(&self) -> Option<&'static str> {
        match self {
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
            _ => "technical error!"
        }.to_string()
    }

}
