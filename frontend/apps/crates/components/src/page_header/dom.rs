use std::{collections::HashMap, rc::Rc};

use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::{UserProfile, UserScope};
use strum::IntoEnumIterator;
use utils::{
    events,
    init::analytics,
    routes::{AdminRoute, AssetRoute, CommunityMembersRoute, CommunityRoute, Route, UserRoute},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use crate::{
    overlay::handle::OverlayHandle,
    page_header::state::{LoggedInState, PageLinks},
};

use super::{actions, PageHeader};

const DONATE_LINK: &str = "https://www.jewishinteractive.org/donate/";

const STR_SIGN_UP: &str = "Sign up";
const STR_LOGIN: &str = "Login";
const STR_LOGOUT: &str = "Logout";
const STR_ADMIN: &str = "Admin";
const STR_DONATE: &str = "Donate";

const STR_MY_SETTINGS: &str = "My settings";
const STR_MY_PROFILE: &str = "My profile";
const STR_MY_JIGS: &str = "My JIGs";
const STR_MY_RESOURCES: &str = "My resources";

impl PageHeader {
    pub fn render(self: Rc<PageHeader>) -> Dom {
        let state = self;
        actions::fetch_profile(Rc::clone(&state));

        html!("page-header", {
            .apply_if(state.config.slot.is_some(), |dom| {
                dom.prop("slot", state.config.slot.unwrap_ji())
            })
            .children(PageLinks::iter().map(|page_link| {
                html!("page-header-link", {
                    .prop("slot", "links")
                    .prop("kind", page_link.kind_str())
                    .prop("active", {
                        matches!(
                            &state.config.active_page,
                            Some(active_page) if active_page == &page_link
                        )
                    })
                    .prop("href", &page_link.route())
                    .prop("target", page_link.target())
                    .event(move |_evt: events::Click| {
                        let mut properties = HashMap::new();
                        properties.insert("Header Kind", page_link.kind_str().to_owned());
                        analytics::event("Header Click", Some(properties));
                    })
                })
            }))
            .child(html!("button-rect", {
                .prop("slot", "donate")
                .prop("color", "green")
                .prop("size", "small")
                .prop("bold", true)
                .prop("href", DONATE_LINK)
                .prop("target", "_blank")
                .text(STR_DONATE)
                .event(move |_evt: events::Click| {
                    analytics::event("Donate Click", None);
                })
            }))
            .child(html!("fa-button", {
                .prop("slot", "help")
                .prop("icon", "fa-regular fa-circle-question fa-5x")
                .event(move |_evt: events::Click| {
                    analytics::event("Help Center Click", None);
                })
            }))
            .apply_if(state.config.render_beta, |dom| {
                dom.child(html!("beta-button", {
                    .prop("slot", "beta")
                    .event(clone!(state => move |_evt: events::Click| {
                        state.beta_tooltip.set_neq(true);
                    }))
                    .with_node!(elem => {
                        .child_signal(state.beta_tooltip.signal_cloned().map(clone!(state, elem => move |show_tooltip| {
                            match show_tooltip {
                                false => None,
                                true => Some(
                                    html!("empty-fragment" => HtmlElement, {
                                        .apply(OverlayHandle::lifecycle(
                                            clone!(state, elem => move || {
                                                html!("overlay-tooltip-info", {
                                                    .prop("target", &elem)
                                                    .prop("color", "light-orange")
                                                    .attr("targetAnchor", "mm")
                                                    .attr("contentAnchor", "bl")
                                                    .prop("closeable", true)
                                                    .prop("strategy", "track")
                                                    .event(clone!(state => move |_evt: events::Close| {
                                                        state.beta_tooltip.set_neq(false);
                                                    }))
                                                    .child(html!("beta-tooltip-content", {
                                                        .prop("slot", "body")
                                                    }))
                                                })
                                            })
                                        ))
                                    })
                                ),
                            }
                        })))
                    })
                }))
            })
            .apply(|dom| {
                if let Some(PageLinks::Home) = state.config.active_page {
                    dom.child(html!("page-header-student-code", {
                        .prop("slot", "student-code")
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
}

fn has_privileges(state: Rc<PageHeader>, scope: UserScope) -> impl Signal<Item = bool> {
    state
        .logged_in
        .signal_ref(move |logged_in_state| {
            matches!(logged_in_state, LoggedInState::LoggedIn(profile) if profile.scopes.contains(&scope))
        })
}

fn render_logged_in(state: Rc<PageHeader>, user: &UserProfile) -> Vec<Dom> {
    vec![html!("page-header-profile", {
        .prop("slot", "user")
        .prop("name", &user.given_name)
        .prop("email", &user.email)
        .children(&mut [
            html!("button-rect", {
                .prop("slot", "logout")
                .prop("kind", "outline")
                .prop("size", "small")
                .prop("color", "blue")
                .text(STR_LOGOUT)
                .event(clone!(state => move |_: events::Click| {
                    actions::logout(Rc::clone(&state));
                    analytics::event("Header Logout Click", None);
                }))
            }),
            html!("profile-image", {
                .prop("slot", "profile-image")
                .prop("imageId", {
                    match &user.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }),
            html!("profile-image", {
                .prop("slot", "overlay-profile-image")
                .prop("imageId", {
                    match &user.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }),
        ])
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href", Route::Asset(AssetRoute::JigGallery).to_string())
            .child(html!("img-ui", {
                .prop("path", "core/page-header/jig-icon.svg")
            }))
            .text(STR_MY_JIGS)
        }))
        .child_signal(has_privileges(Rc::clone(&state), UserScope::Admin).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .prop("slot", "user-links")
                        .prop("href", Route::Asset(AssetRoute::ResourceGallery).to_string())
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-light fa-lightbulb-on")
                        }))
                        .text(STR_MY_RESOURCES)
                    }))
                }
            }
        }))
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href", Route::User(UserRoute::Settings).to_string())

            .child(html!("fa-icon", {
                .prop("icon", "fa-light fa-gear")
            }))
            .text(STR_MY_SETTINGS)
        }))
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href",  Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(user.id))).to_string())

            .child(html!("fa-icon", {
                .prop("icon", "fa-light fa-user")
            }))
            .text(STR_MY_PROFILE)
        }))
        .child_signal(has_privileges(Rc::clone(&state), UserScope::Admin).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .prop("slot", "admin")
                        .prop("href", Route::Admin(AdminRoute::Landing).to_string())
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
            .prop("slot", "user")
            .prop("kind", "text")
            .prop("color", "black")
            .prop("href", &Route::User(UserRoute::Register(Default::default())).to_string())
            .text(STR_SIGN_UP)
            .event(move |_evt: events::Click| {
                analytics::event("Header Signup Click", None);
            })
        }),
        html!("button-rect", {
            .prop("slot", "user")
            .prop("kind", "text")
            .prop("color", "black")
            .prop("href", &Route::User(UserRoute::Login(Default::default())).to_string())
            .text(STR_LOGIN)
            .event_with_options(
                &EventOptions::preventable(),
                |e: events::Click| {
                    e.prevent_default();

                    actions::navigate_to_login();
                    analytics::event("Header Login Click", None);
                }
            )
        }),
    ]
}
