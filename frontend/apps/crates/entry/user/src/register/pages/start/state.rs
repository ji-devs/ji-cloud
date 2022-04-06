use crate::{
    email_handler::EmailHandler, password_handler::PasswordHandler, register::state::Step,
};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use utils::routes::RegisterQuery;
use std::rc::Rc;

pub struct RegisterStart {
    pub loader: AsyncLoader,
    pub email: EmailHandler,
    pub password: PasswordHandler,
    pub step: Mutable<Step>,
    pub tried_to_submit: Mutable<bool>,
    pub login_before_register: bool,
}

impl RegisterStart {
    // _is_no_auth check's if user was logged out
    pub fn new(step: Mutable<Step>, query: RegisterQuery) -> Rc<Self> {

        Rc::new(Self {
            loader: AsyncLoader::new(),
            email: EmailHandler::new(),
            password: PasswordHandler::new(),
            step,
            tried_to_submit: Mutable::new(false),
            login_before_register: query.login_before_register,
        })
    }

    pub fn show_email_error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        map_ref! {
            let error = self.email.error_signal(),
            let tried_to_submit = self.tried_to_submit.signal() => move {
                if error.is_some() && *tried_to_submit {
                    Some(error.unwrap())
                } else {
                    None
                }
            }
        }
    }

    pub fn show_password_error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
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
