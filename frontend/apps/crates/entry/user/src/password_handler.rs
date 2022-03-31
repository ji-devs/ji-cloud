use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::cell::RefCell;
use utils::unwrap::UnwrapJiExt;

use zxcvbn::{zxcvbn, Entropy};

const STR_PASSWORD_SHORT: &str = "Password can't be empty";
const STR_PASSWORD_EMPTY: &str = "Your password should be at least 6 characters.";

pub struct PasswordHandler {
    pub(super) strength: Mutable<PasswordStrength>,
    pub(super) value: RefCell<String>,
    pub(super) error: Mutable<Option<&'static str>>,
}

impl PasswordHandler {
    pub fn new() -> Self {
        let _self = Self {
            strength: Mutable::new(PasswordStrength::None),
            value: RefCell::new("".to_string()),
            error: Mutable::new(None),
        };
        _self.update_errors();
        _self.update_strength();
        _self
    }

    pub fn password_acceptable(&self) -> bool {
        self.error.lock_ref().is_none()
    }

    pub fn update_value(&self, value: String) {
        *self.value.borrow_mut() = value;
        self.update_errors();
        self.update_strength();
    }

    pub fn set_error(&self, error: &'static str) {
        self.error.set(Some(error));
    }

    pub fn get_value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn strength_signal(&self) -> impl Signal<Item = &'static str> {
        self.strength.signal().map(|x| x.as_str())
    }

    pub fn error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        self.error.signal_cloned()
    }

    fn update_errors(&self) {
        let password = &self.value.borrow();
        let error = if password.is_empty() {
            Some(STR_PASSWORD_EMPTY)
        } else if password.len() < 6 {
            Some(STR_PASSWORD_SHORT)
        } else {
            None
        };
        self.error.set(error);
    }

    fn update_strength(&self) {
        let password = &self.value.borrow();
        if password.is_empty() {
            self.strength.set(PasswordStrength::None);
        } else {
            let estimate = zxcvbn(password, &[]).unwrap_ji();
            self.strength.set(estimate.into());
        }
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
