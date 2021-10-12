use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::register::state::Step;

const STR_SUBMIT:&'static str = "Submit";
const STR_EMAIL_LABEL:&'static str = "Email";
const STR_EMAIL_PLACEHOLDER:&'static str = "Type or paste your email";
const STR_PASSWORD_LABEL:&'static str = "Create Password";
const STR_PASSWORD_PLACEHOLDER:&'static str ="********";

pub struct Footer {
}

impl Footer {
    pub fn render() -> Dom {
        html!("footer-register-login", {
            .property("slot", "footer")
            .event(|evt:events::Click| {
                go_login()
            })
        })
    }
}

fn go_login() {
    let route:String = Route::User(UserRoute::Login(String::new())).into();
    dominator::routing::go_to_url(&route);
}
