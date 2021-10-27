use crate::{password::state::PasswordState, register::state::Step};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;

pub struct State {
    pub loader: AsyncLoader,
    pub email: RefCell<String>,
    pub email_status: Mutable<Option<EmailStatus>>,
    pub password: PasswordState,
    pub step: Mutable<Step>,
}

impl State {
    pub fn new(step: Mutable<Step>) -> Self {
        Self {
            loader: AsyncLoader::new(),
            email: RefCell::new("".to_string()),
            email_status: Mutable::new(None),
            password: PasswordState::new(),
            step,
        }
    }

    pub fn clear_email_status(&self) {
        self.email_status.set(None);
    }

    pub fn email_error(&self) -> impl Signal<Item = &'static str> {
        self.email_status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }
}

#[derive(Debug, Clone)]
pub enum EmailStatus {
    // ConfirmEmail,
    EmptyEmail,
    InvalidEmail,
    // IdExists,
    EmailExists,
    // UnknownFirebase,
    // Technical,
}

impl EmailStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Self::ConfirmEmail => "confirm your email!",
            Self::EmptyEmail => "supply an email address!",
            Self::InvalidEmail => "invalid email address",
            // Self::IdExists => "id exists!",
            Self::EmailExists => "Email in use!",
            // Self::UnknownFirebase => "firebase error!",
            // Self::Technical => "technical error!",
        }
    }
}
