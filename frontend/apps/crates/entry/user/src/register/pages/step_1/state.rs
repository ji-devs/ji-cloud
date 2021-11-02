use crate::register::state::Step;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::session::OAuthUserProfile;
use std::cell::RefCell;

pub struct State {
    pub step: Mutable<Step>,
    pub username_taken_loader: AsyncLoader,
    pub firstname: RefCell<String>,
    pub firstname_status: Mutable<Option<NameError>>,
    pub lastname: RefCell<String>,
    pub lastname_status: Mutable<Option<NameError>>,
    pub username: RefCell<String>,
    pub username_status: Mutable<Option<NameError>>,
    pub over_18: RefCell<bool>,
    pub over_18_status: Mutable<Option<Over18Error>>,
    pub oauth_profile: Option<OAuthUserProfile>,
}

impl State {
    pub fn new(step: Mutable<Step>, oauth_profile: Option<OAuthUserProfile>) -> Self {
        let (firstname, lastname) = match oauth_profile.clone() {
            None => (RefCell::new("".to_string()), RefCell::new("".to_string())),
            Some(oauth_profile) => (
                RefCell::new(oauth_profile.given_name.unwrap_or_default()),
                RefCell::new(oauth_profile.family_name.unwrap_or_default()),
            ),
        };

        Self {
            step,
            oauth_profile,
            firstname,
            lastname,
            username: RefCell::new("".to_string()),
            username_taken_loader: AsyncLoader::new(),
            firstname_status: Mutable::new(None),
            lastname_status: Mutable::new(None),
            username_status: Mutable::new(None),
            over_18: RefCell::new(false),
            over_18_status: Mutable::new(None),
        }
    }

    pub fn clear_firstname_status(&self) {
        self.firstname_status.set(None);
    }
    pub fn clear_lastname_status(&self) {
        self.lastname_status.set(None);
    }
    pub fn clear_username_status(&self) {
        self.username_status.set(None);
    }
    pub fn clear_over_18_status(&self) {
        self.over_18_status.set(None);
    }

    pub fn firstname_error(&self) -> impl Signal<Item = &'static str> {
        self.firstname_status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }

    pub fn lastname_error(&self) -> impl Signal<Item = &'static str> {
        self.lastname_status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }

    pub fn username_error(&self) -> impl Signal<Item = &'static str> {
        self.username_status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }

    pub fn over_18_error(&self) -> impl Signal<Item = &'static str> {
        self.over_18_status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }
}

#[derive(Debug, Clone)]
pub enum NameError {
    Empty,
    Exists,
}

impl NameError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Empty => "Can't be empty!",
            Self::Exists => "Already exists!",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Over18Error {
    Unchecked,
}

impl Over18Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unchecked => "You must be at least 18 to register with Jigzi.",
        }
    }
}
