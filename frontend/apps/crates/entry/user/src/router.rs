use utils::routes::*;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use crate::{
    login::dom::LoginPage,
    profile::dom::ProfilePage,
    register::pages::start::dom::StartPage as RegisterPage,
};

pub struct Router {
}

impl Router {
    pub fn new() -> Self {
        Self { }
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal()
            .map(|route| {
                match route {
                    Route::User(route) => {
                        match route {
                            UserRoute::Login => Some(LoginPage::render()),
                            UserRoute::Profile(ProfileSection::Landing) => Some(ProfilePage::render()),
                            UserRoute::Register => Some(RegisterPage::render()),
                            /*
                            UserRoute::ContinueRegistration(user) => Some(RegisterPage::render(RegisterPage::new(Some(user)))),
                            UserRoute::Profile(ProfileSection::ChangeEmail) => Some(ProfileEmailChangePage::render(ProfileEmailChangePage::new())),
                            UserRoute::SendEmailConfirmation => Some(SendEmailConfirmationPage::render(SendEmailConfirmationPage::new())),
                            UserRoute::GotEmailConfirmation => Some(GotEmailConfirmationPage::render(GotEmailConfirmationPage::new())),
                            */
                            _ => None
                        }
                    }
                    _ => None
                }
            })
    }
    
    pub fn render(&self) -> Dom {
        html!("main", { .child_signal(Self::dom_signal()) } )
    }
}
