use std::{collections::HashMap, rc::Rc};

use dominator::{clone, html, Dom, EventOptions};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::{UserProfile, UserScope};
use strum::IntoEnumIterator;
use utils::window::navigate_to_login;
use utils::{
    events,
    init::analytics,
    paywall,
    routes::{
        AdminRoute, AssetRoute, ClassroomCodesRoute, ClassroomRoute, CommunityMembersRoute,
        CommunityRoute, HomePricingRoute, HomeRoute, Route, UserRoute,
    },
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;

use crate::page_header::state::{LoggedInState, PageLinks};

use super::{actions, PageHeader};

const STR_SIGN_UP: &str = "Sign up";
const STR_LOGIN: &str = "Login";
const STR_LOGOUT: &str = "Logout";
const STR_ADMIN: &str = "Admin";
const STR_PRICING: &str = "Plans";
const STR_ACCOUNT: &str = "Account";
const STR_PROFILE: &str = "Profile";
const STR_MY_JIGS: &str = "My JIGs";
const STR_MY_PLAYLISTS: &str = "My playlists";
// const STR_MY_COURSES: &str = "My courses";
const STR_MY_RESOURCES: &str = "My resources";
const STR_CLASSES: &str = "My classes";

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
                .prop("size", "large")
                .prop("bold", true)
                .prop("href", Route::Home(HomeRoute::Pricing(HomePricingRoute::Individual)).to_string())
                // .prop("href", DONATE_LINK)
                // .prop("target", "_blank")
                .text(STR_PRICING)
                .event(move |_evt: events::Click| {
                    analytics::event("Pricing Click", None);
                })
            }))
            .child(html!("div", {
                .prop("slot", "help")
                .child(html!("img-ui", {
                    .prop("path", "core/page-header/icon-help.svg")
                    .style("width", "20px")
                    .style("height", "20px")
                }))
                .event(move |_evt: events::Click| {
                    analytics::event("Help Center Click", None);
                })
            }))
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
                .prop("size", "regular")
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
                .prop("givenName", &user.given_name)
                .prop("familyName", &user.family_name)
            }),
            html!("profile-image", {
                .prop("slot", "overlay-profile-image")
                .prop("imageId", {
                    match &user.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
                .prop("givenName", &user.given_name)
                .prop("familyName", &user.family_name)
            }),
        ])
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href",  Route::Asset(AssetRoute::JigGallery).to_string())
            .prop("target", "_top")
            .child(html!("img-ui", {
                .prop("path", "core/page-header/jig-icon.svg")
            }))
            .text(STR_MY_JIGS)
        }))
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href", Route::Asset(AssetRoute::PlaylistGallery).to_string())
            .prop("target", "_top")
            .child(html!("img-ui", {
                .prop("path", "core/page-header/nav-icon-assets.svg")
            }))
            .text(STR_MY_PLAYLISTS)
        }))
        // .child(html!("a", {
        //     .prop("slot", "user-links")
        //     .prop("href", Route::Asset(AssetRoute::CourseGallery).to_string())
        //     .prop("target", "_top")
        //     .child(html!("img-ui", {
        //         .prop("path", "core/page-header/nav-icon-course.svg")
        //     }))
        //     .text(STR_MY_COURSES)
        // }))
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href", Route::Asset(AssetRoute::ResourceGallery).to_string())
            .prop("target", "_top")
            .child(html!("img-ui", {
                .prop("path", "core/page-header/nav-icon-resource.svg")
            }))
            .text(STR_MY_RESOURCES)
        }))
        .child(html!("a", {
            .prop("slot", "user-links")
            .prop("href", Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::Jigs)).to_string())
            .prop("target", "_top")
            .child(html!("img-ui", {
                .prop("path", "core/page-header/nav-icon-classes.svg")
            }))
            .text(STR_CLASSES)
            .event_with_options(&EventOptions::preventable(), |e: events::Click| {
                if !paywall::can_create_codes() {
                    e.prevent_default();
                    paywall::dialog_limit("
                        Looking to use our scoring and tracking functionality?
                        Upgrade now for UNLIMITED sharing options.
                    ");
                }
            })
        }))
        .child(html!("a", {
            .prop("slot", "setting-links")
            .prop("href",  Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(user.id))).to_string())
            .prop("target", "_top")
            .child(html!("fa-icon", {
                .prop("icon", "fa-light fa-user")
            }))
            .text(STR_PROFILE)
        }))
        .child(html!("a", {
            .prop("slot", "setting-links")
            .prop("href", Route::User(UserRoute::Settings).to_string())
            .prop("target", "_top")
            .child(html!("fa-icon", {
                .prop("icon", "fa-light fa-gear")
            }))
            .text(STR_ACCOUNT)
        }))
        .child_signal(has_privileges(Rc::clone(&state), UserScope::Admin).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .prop("slot", "admin")
                        .prop("href", Route::Admin(AdminRoute::Landing).to_string())
                        .prop("target", "_top")
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

                    navigate_to_login();
                    analytics::event("Header Login Click", None);
                }
            )
        }),
    ]
}
