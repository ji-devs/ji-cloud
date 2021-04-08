use utils::{storage, routes::*};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use dominator::{Dom, html};
use shared::domain::session::{CreateSessionResponse, OAuthUrlKind};
use crate::{
    register::dom::RegisterPage,
    register::pages::complete::dom::CompletePage as RegisterCompletePage,
    register::state::Step,
    oauth::dom::OauthPage,
    login::dom::LoginPage,
    profile::dom::ProfilePage,
    //register_complete::dom::RegisterCompletePage,
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
                            UserRoute::Register => Some(RegisterPage::render(None)),
                            UserRoute::RegisterOauth(data) => Some(OauthPage::render(data, OAuthUrlKind::Register)),
                            UserRoute::LoginOauth(data) => Some(OauthPage::render(data, OAuthUrlKind::Login)),
                            UserRoute::Login => Some(LoginPage::render()),
                            UserRoute::Profile(ProfileSection::Landing) => Some(ProfilePage::render()),
                            UserRoute::RegisterComplete => Some(RegisterCompletePage::render()),
                            UserRoute::ContinueRegistration => Some(RegisterPage::render(Some(Step::One))),
                            /*
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
