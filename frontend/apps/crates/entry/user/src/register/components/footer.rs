#![allow(dead_code)] // this should be remove eventually

use dominator::{html, Dom};

use utils::{events, routes::*};

const STR_SUBMIT: &str = "Submit";
const STR_EMAIL_LABEL: &str = "Email";
const STR_EMAIL_PLACEHOLDER: &str = "Type or paste your email";
const STR_PASSWORD_LABEL: &str = "Create Password";
const STR_PASSWORD_PLACEHOLDER: &str = "********";

pub struct Footer {}

impl Footer {
    pub fn render() -> Dom {
        html!("footer-register-login", {
            .property("slot", "footer")
            .event(|_evt:events::Click| {
                go_login()
            })
        })
    }
}

fn go_login() {
    let route: String = Route::User(UserRoute::Login(String::new())).into();
    dominator::routing::go_to_url(&route);
}
