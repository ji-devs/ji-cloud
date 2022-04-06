use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use utils::routes::LoginQuery;
use std::{cell::RefCell, rc::Rc};

use crate::email_handler::EmailHandler;

pub struct LoginPage {
    pub loader: AsyncLoader,
    pub email: EmailHandler,
    pub password: RefCell<String>,
    pub password_error: Mutable<Option<&'static str>>,
    pub reset_password_popup: Mutable<bool>,
    pub tried_to_submit: Mutable<bool>,
    pub basic_tried_oauth: bool,
}
impl LoginPage {
    pub fn new(query: LoginQuery) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            email: EmailHandler::new(),
            password: RefCell::new("".to_string()),
            password_error: Mutable::new(None),
            reset_password_popup: Mutable::new(false),
            tried_to_submit: Mutable::new(false),
            basic_tried_oauth: query.basic_tried_oauth,
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
}
