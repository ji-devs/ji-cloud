use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;

use zxcvbn::Entropy;

pub struct PasswordState {
    pub strength: Mutable<PasswordStrength>,
    pub value: RefCell<String>,
    pub status: Mutable<Option<PasswordStatus>>,
}

impl PasswordState {
    pub fn new() -> Self {
        Self {
            strength: Mutable::new(PasswordStrength::None),
            value: RefCell::new("".to_string()),
            status: Mutable::new(None),
        }
    }

    pub fn get_strength(&self) -> impl Signal<Item = &'static str> {
        self.strength.signal().map(|x| x.as_str())
    }

    pub fn clear_status(&self) {
        self.status.set(None);
    }

    pub fn error(&self) -> impl Signal<Item = &'static str> {
        self.status
            .signal_cloned()
            .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PasswordStrength {
    None,
    Weak,
    Average,
    Strong,
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
    fn from(entropy: Entropy) -> Self {
        if crate::debug::settings().skip_password_strength {
            Self::Strong
        } else {
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
}

#[derive(Debug, Clone)]
pub enum PasswordStatus {
    EmptyPw,
    PwMismatch,
    PwWeak,
    PwShort,
    ResetError,
    UnknownFirebase,
    Technical,
}

impl PasswordStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPw => "supply a password!",
            Self::PwMismatch => "passwords don't match!",
            Self::PwWeak => "weak password!",
            Self::PwShort => "Your password should be at least 6 characters.",
            Self::UnknownFirebase => "firebase error!",
            Self::ResetError => "unable to reset password!",
            Self::Technical => "technical error!",
        }
    }
}
