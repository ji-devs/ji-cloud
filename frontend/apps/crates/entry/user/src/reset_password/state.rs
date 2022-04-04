use std::rc::Rc;

use super::super::password_handler::PasswordHandler;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};

pub struct PasswordResetPage {
    pub token: String,
    pub loader: AsyncLoader,
    pub password: PasswordHandler,
    pub tried_to_submit: Mutable<bool>,
}

impl PasswordResetPage {
    pub fn new(token: String) -> Rc<Self> {
        Rc::new(Self {
            token,
            loader: AsyncLoader::new(),
            password: PasswordHandler::new(),
            tried_to_submit: Mutable::new(false),
        })
    }

    pub fn show_error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        map_ref! {
            let error = self.password.error_signal(),
            let tried_to_submit = self.tried_to_submit.signal() => move {
                if error.is_some() && *tried_to_submit {
                    Some(error.unwrap())
                } else {
                    None
                }
            }
        }
    }
}
