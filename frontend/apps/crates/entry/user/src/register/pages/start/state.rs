use crate::{password_handler::PasswordHandler, register::state::Step};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use std::{cell::RefCell, rc::Rc};

pub struct RegisterStart {
    pub loader: AsyncLoader,
    pub email: RefCell<String>,
    // pub email_status: Mutable<Option<EmailStatus>>,
    pub email_error: Mutable<Option<&'static str>>,
    pub password: PasswordHandler,
    pub step: Mutable<Step>,
    pub tried_to_submit: Mutable<bool>,
}

impl RegisterStart {
    // _is_no_auth check's if user was logged out
    pub fn new(step: Mutable<Step>, _is_no_auth: bool) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            email: RefCell::new("".to_string()),
            email_error: Mutable::new(None),
            password: PasswordHandler::new(),
            step,
            tried_to_submit: Mutable::new(false),
        })
    }

    // pub fn clear_email_status(&self) {
    //     self.email_status.set(None);
    // }

    // pub fn email_error(&self) -> impl Signal<Item = &'static str> {
    //     self.email_status
    //         .signal_cloned()
    //         .map(|err| err.map(|err| err.as_str()).unwrap_or(""))
    // }

    pub fn show_email_error_signal(&self) -> impl Signal<Item = Option<&'static str>> {
        map_ref! {
            let error = self.email_error.signal(),
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

// #[derive(Debug, Clone)]
// pub enum EmailStatus {
//     // ConfirmEmail,
//     EmptyEmail,
//     InvalidEmail,
//     // IdExists,
//     EmailExists,
//     // UnknownFirebase,
//     // Technical,
// }

// impl EmailStatus {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             // Self::ConfirmEmail => "confirm your email!",
//             Self::EmptyEmail => "supply an email address!",
//             Self::InvalidEmail => "invalid email address",
//             // Self::IdExists => "id exists!",
//             Self::EmailExists => "Email in use!",
//             // Self::UnknownFirebase => "firebase error!",
//             // Self::Technical => "technical error!",
//         }
//     }
// }
