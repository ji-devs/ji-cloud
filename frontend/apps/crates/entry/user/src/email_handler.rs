use futures_signals::signal::{Mutable, Signal};
use std::cell::RefCell;

const STR_EMAIL_INVALID: &str = "Invalid email";
const STR_EMAIL_EMPTY: &str = "Email address can't be empty";

pub struct EmailHandler {
    pub(super) value: RefCell<String>,
    pub(super) error: Mutable<Option<&'static str>>,
}

impl EmailHandler {
    pub fn new() -> Self {
        let _self = Self {
            value: RefCell::new("".to_string()),
            error: Mutable::new(None),
        };
        _self.update_errors();
        _self
    }

    pub fn email_acceptable(&self) -> bool {
        self.error.lock_ref().is_none()
    }

    pub fn email_acceptable_signal(&self) -> impl Signal<Item = bool> {
        self.error.signal_ref(|e| e.is_none())
    }

    pub fn update_value(&self, value: String) {
        *self.value.borrow_mut() = value;
        self.update_errors();
    }

    pub fn set_error(&self, error: &'static str) {
        self.error.set(Some(error));
    }

    pub fn get_value(&self) -> String {
        self.value.borrow().clone()
    }

    pub fn error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        self.error.signal_cloned()
    }

    fn update_errors(&self) {
        let email = &self.value.borrow();
        let error = if email.is_empty() {
            Some(STR_EMAIL_EMPTY)
        } else if !email.contains('@') {
            Some(STR_EMAIL_INVALID)
        } else {
            None
        };
        self.error.set(error);
    }
}
