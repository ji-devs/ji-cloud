use std::rc::Rc;

use dominator::{clone, html, Dom, EventOptions};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::{UserProfile, UserScope};
use strum::IntoEnumIterator;
use utils::{
    events,
    routes::{AdminRoute, Route, UserRoute},
};
use wasm_bindgen::JsValue;

use crate::page_header::state::{LoggedInState, PageLinks};

use super::{actions, state::State};

const DONATE_LINK: &str = "https://www.jewishinteractive.org/donate/";

const STR_SIGN_UP: &str = "Sign up";
const STR_LOGIN: &str = "Login";
const STR_LOGOUT: &str = "Logout";
const STR_ADMIN: &str = "Admin";
const STR_DONATE: &str = "Donate";

const STR_MY_PROFILE: &str = "My profile";
const STR_MY_JIGS: &str = "My JIGs";
const STR_MY_RESOURCES: &str = "My resources";

pub fn render(state: Rc<State>, slot: Option<&str>, active_page: Option<PageLinks>) -> Dom {
    actions::fetch_profile(Rc::clone(&state));

    html!("page-header", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap())
        })
        .children(PageLinks::iter().map(|page_link| {
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", page_link.kind_str())
                .property("active", match &active_page {
                    Some(active_page) if active_page == &page_link => true,
                    _ => false,
                })
                .property("href", &page_link.route())
                .property("target", page_link.target())
            })
        }))
        .child(html!("button-rect", {
            .property("slot", "donate")
            .property("color", "green")
            .property("size", "small")
            .property("bold", true)
            .property("href", DONATE_LINK)
            .property("target", "_black")
            .text(STR_DONATE)
        }))
        .apply(|dom| {
            if let Some(PageLinks::Home) = active_page {
                dom.child(html!("page-header-student-code", {
                    .property("slot", "student-code")
                }))
            } else {
                dom
            }

        })
        .children_signal_vec(state.logged_in.signal_cloned().map(clone!(state => move|logged_in| {
            match logged_in {
                LoggedInState::LoggedIn(user) => render_logged_in(Rc::clone(&state), &user),
                LoggedInState::LoggedOut => render_logged_out(),
                LoggedInState::Loading => vec![],
            }
        })).to_signal_vec())
    })
}

fn has_privileges(state: Rc<State>, scope: UserScope) -> impl Signal<Item = bool> {
    state
        .logged_in
        .signal_ref(move |logged_in_state| match logged_in_state {
            LoggedInState::LoggedIn(profile) if profile.scopes.contains(&scope) => true,
            _ => false,
        })
}

fn render_logged_in(state: Rc<State>, user: &UserProfile) -> Vec<Dom> {
    vec![html!("page-header-profile", {
        .property("slot", "user")
        .property("name", &user.given_name)
        .property("email", &user.email)
        .children(&mut [
            html!("button-rect", {
                .property("slot", "logout")
                .property("kind", "outline")
                .property("size", "small")
                .property("color", "blue")
                .text(STR_LOGOUT)
                .event(clone!(state => move |_: events::Click| {
                    actions::logout(Rc::clone(&state));
                }))
            }),
            html!("profile-image", {
                .property("slot", "profile-image")
                .property("imageId", {
                    match &user.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }),
            html!("profile-image", {
                .property("slot", "overlay-profile-image")
                .property("imageId", {
                    match &user.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }),
        ])
        .child(html!("a", {
            .property("slot", "user-links")
            .property("href", "/jig/edit/gallery")
            .child(html!("img-ui", {
                .property("path", "core/page-header/jig-icon.svg")
            }))
            .text(STR_MY_JIGS)
        }))
        .child_signal(has_privileges(Rc::clone(&state), UserScope::Admin).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .property("slot", "user-links")
                        .property("href", "/jig/edit/resource-gallery")
                        .child(html!("fa-icon", {
                            .property("icon", "fa-light fa-lightbulb-on")
                        }))
                        .text(STR_MY_RESOURCES)
                    }))
                }
            }
        }))
        .child(html!("a", {
            .property("slot", "user-links")
            .property("href", "/user/profile")
            .child(html!("fa-icon", {
                .property("icon", "fa-light fa-gear")
            }))
            .text(STR_MY_PROFILE)
        }))
        .child_signal(has_privileges(Rc::clone(&state), UserScope::Admin).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .property("slot", "admin")
                        .property("href", {
                            let url: String = Route::Admin(AdminRoute::Landing).into();
                            url
                        })
                        .text(STR_ADMIN)
                    }))
                }
            }
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
            .property("href", &Route::User(UserRoute::Login(String::new())).to_string())
            .text(STR_LOGIN)
            .event_with_options(
                &EventOptions::preventable(),
                |e: events::Click| {
                    e.prevent_default();

                    actions::navigate_to_login();
                }
            )
        }),
    ]
}
