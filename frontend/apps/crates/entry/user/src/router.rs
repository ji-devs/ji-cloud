use components::overlay::container::OverlayContainer;
use utils::{component::Component, routes::*};

use crate::{
    email::confirmation::SendEmailConfirmationPage,
    email::verify::VerifyEmailPage,
    login::LoginPage,
    oauth::dom::OauthPage,
    register::{dom::RegisterPage, state::Step},
    reset_password::PasswordResetPage,
    school_end::SchoolEnd,
    school_start::SchoolStart,
    settings::state::SettingsPage,
    subscribe1::Subscribe1,
    subscribe2::Subscribe2,
    welcome::Welcome,
};
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::session::OAuthUrlKind;

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal().map(|route| match route {
            Route::User(route) => match route {
                UserRoute::Register(query) => Some(RegisterPage::render(None, query)),
                UserRoute::NoAuth => Some(LoginPage::new(Default::default()).render()),
                UserRoute::RegisterOauth(data) => {
                    Some(OauthPage::render(data, OAuthUrlKind::Register))
                }
                UserRoute::LoginOauth(data) => Some(OauthPage::render(data, OAuthUrlKind::Login)),
                UserRoute::Login(query) => Some(LoginPage::new(query).render()),
                UserRoute::Settings => Some(SettingsPage::new().render()),
                UserRoute::ContinueRegistration(oauth_profile) => Some(RegisterPage::render(
                    Some(Step::One(oauth_profile)),
                    Default::default(),
                )),
                UserRoute::SendEmailConfirmation(email) => Some(SendEmailConfirmationPage::render(
                    SendEmailConfirmationPage::new(email),
                )),
                UserRoute::VerifyEmail(token) => {
                    Some(VerifyEmailPage::render(VerifyEmailPage::new(token)))
                }
                UserRoute::PasswordReset(token) => {
                    Some(PasswordResetPage::render(PasswordResetPage::new(token)))
                }
                UserRoute::SchoolStart(plan_type) => {
                    SchoolStart::new(plan_type).map(|state| state.render())
                }
                UserRoute::SchoolEnd => Some(SchoolEnd::new().render()),
                UserRoute::Subscribe1(plan_type) => Some(Subscribe1::new(plan_type).render()),
                UserRoute::Subscribe2(plan_type) => Some(Subscribe2::new(plan_type).render()),
                UserRoute::Welcome => Some(Welcome::new().render()),
            },
            _ => None,
        })
    }

    pub fn render(&self) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }
}
