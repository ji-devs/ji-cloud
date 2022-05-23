use futures_signals::signal::{Mutable, Signal};

const STR_EMAIL_INVALID: &str = "Invalid email";
const STR_EMAIL_EMPTY: &str = "Email address can't be empty";

pub struct EmailHandler {
    pub(super) value: Mutable<String>,
    pub(super) error: Mutable<Option<&'static str>>,
}

impl EmailHandler {
    pub fn new() -> Self {
        let _self = Self {
            value: Mutable::new(String::new()),
            error: Mutable::new(None),
        };
        _self.update_errors();
        _self
    }

    pub fn email_acceptable(&self) -> bool {
        let email = &self.value.lock_ref();
        get_error(email).is_none()
    }

    pub fn email_acceptable_signal(&self) -> impl Signal<Item = bool> {
        self.value.signal_ref(|e| get_error(&*e).is_none())
    }

    pub fn update_value(&self, mut value: String) {
        value = value.trim().to_string();
        self.value.set(value);
        self.update_errors();
    }

    pub fn set_error(&self, error: &'static str) {
        self.error.set(Some(error));
    }

    pub fn get_value(&self) -> String {
        self.value.get_cloned()
    }

    pub fn error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        self.error.signal_cloned()
    }

    fn update_errors(&self) {
        let email = &self.value.lock_ref();
        let error = get_error(email);
        self.error.set(error);
    }
}

fn get_error(email: &str) -> Option<&'static str> {
    if email.is_empty() {
        Some(STR_EMAIL_EMPTY)
    } else if !valid_email(email) {
        Some(STR_EMAIL_INVALID)
    } else {
        None
    }
}

fn valid_email(email: &str) -> bool {
    email.contains('@') && !email.contains(' ')
}
