use std::rc::Rc;

use dominator::{html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::user::UserProfile;
use utils::routes::{JigRoute, ProfileSection, Route, UserRoute};

use crate::page_header::state::LoggedInState;

use super::{actions, state::State};

const STR_SIGN_UP: &'static str = "Sign up";
const STR_LOGIN: &'static str = "Login";

pub fn render(state: Rc<State>) -> Dom {
    actions::fetch_profile(Rc::clone(&state));

    html!("page-header", {
        .children(&mut [
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "home")
                .property("active", true)
                .property("href", &Route::Home.to_string())
            }),
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "content")
            }),
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "create")
                .property("href", &Route::Jig(JigRoute::Gallery).to_string())
            }),
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "community")
            }),
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "classroom")
            }),
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", "about")
            }),
            html!("button-rect", {
                .property("slot", "donate")
                .property("color", "green")
                .property("size", "small")
                .property("bold", "")
                .text("Donate")
            }),
        ])
        .children_signal_vec(state.logged_in.signal_cloned().map(|logged_in| {
            match logged_in {
                LoggedInState::LoggedIn(user) => render_logged_in(&user),
                LoggedInState::LoggedOut => render_logged_out(),
                LoggedInState::Loading => vec![],
            }
        }).to_signal_vec())
    })
}

fn render_logged_in(user: &UserProfile) -> Vec<Dom> {
    vec![html!("a", {
        .property("slot", "user")
        .property("href", &Route::User(UserRoute::Profile(ProfileSection::Landing)).to_string())
        .style("text-decoration", "none")
        .child(html!("page-header-profile", {
            .style("cursor", "pointer")
            .property("name", &user.given_name)
        }))
    })]
}

fn render_logged_out() -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .property("slot", "user")
            .property("kind", "text")
            .property("color", "black")
            .property("href", &Route::User(UserRoute::Register).to_string())
            .text(STR_SIGN_UP)
        }),
        html!("button-rect", {
            .property("slot", "user")
            .property("kind", "text")
            .property("color", "black")
            .property("href", &Route::User(UserRoute::Login).to_string())
            .text(STR_LOGIN)
        }),
    ]
}
