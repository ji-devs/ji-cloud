use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use std::cell::RefCell;
use crate::register::state::Step;
use zxcvbn::Entropy;

pub struct State {
    pub loader: AsyncLoader,
    pub password_strength: Mutable<PasswordStrength>,
    pub email: RefCell<String>,
    pub email_status: Mutable<Option<EmailStatus>>,
    pub password: RefCell<String>,
    pub password_status: Mutable<Option<PasswordStatus>>,
    pub step: Mutable<Step>,
}

impl State {
    pub fn new(step: Mutable<Step>) -> Self {
        Self {
            loader: AsyncLoader::new(),
            password_strength: Mutable::new(PasswordStrength::None),
            email: RefCell::new("".to_string()),
            email_status: Mutable::new(None),
            password: RefCell::new("".to_string()),
            password_status: Mutable::new(None),
            step
        }
    }

    pub fn get_password_strength(&self) -> impl Signal<Item = &'static str> {
        self.password_strength
            .signal()
            .map(|x| x.as_str())
    }

    pub fn clear_email_status(&self) {
        self.email_status.set(None);
    }
    pub fn clear_password_status(&self) {
        self.password_status.set(None);
    }

    pub fn email_error(&self) -> impl Signal<Item = &'static str> {
        self.email_status
            .signal_cloned()
            .map(|err| {
                err
                    .map(|err| err.as_str())
                    .unwrap_or("")
            })
    }

    pub fn password_error(&self) -> impl Signal<Item = &'static str> {
        self.password_status
            .signal_cloned()
            .map(|err| {
                err
                    .map(|err| err.as_str())
                    .unwrap_or("")
            })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PasswordStrength {
    None,
    Weak,
    Average,
    Strong
}

impl PasswordStrength {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Weak => "weak",
            Self::Average => "average",
            Self::Strong => "strong",
        }
    }
}

impl From<Entropy> for PasswordStrength {
    fn from(entropy:Entropy) -> Self {
        let score = entropy.score();
        if score < 2 {
            Self::Weak
        } else if score < 4 {
            Self::Average
        } else {
            Self::Strong
        }
    }
}

#[derive(Debug, Clone)]
pub enum EmailStatus {
    ConfirmEmail,
    EmptyEmail,
    InvalidEmail,
    IdExists,
    EmailExists,
    UnknownFirebase,
    Technical 
}

#[derive(Debug, Clone)]
pub enum PasswordStatus {
    EmptyPw,
    PwMismatch,
    PwWeak,
    UnknownFirebase,
    Technical 
}

impl EmailStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConfirmEmail => "confirm your email!",
            Self::EmptyEmail => "supply an email address!",
            Self::InvalidEmail => "invalid email address!",
            Self::IdExists => "id exists!",
            Self::EmailExists => "Email in use!",
            Self::UnknownFirebase => "firebase error!",
            Self::Technical => "technical error!",
        }
    }
}
impl PasswordStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPw => "supply a password!",
            Self::PwMismatch => "passwords don't match!",
            Self::PwWeak => "weak password!",
            Self::UnknownFirebase => "firebase error!",
            Self::Technical => "technical error!",
        }
    }
}
