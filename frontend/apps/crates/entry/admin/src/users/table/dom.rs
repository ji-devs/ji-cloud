use crate::users::editable_user::EditableUser;

use super::state::*;
use dominator::{clone, html, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::user::UserBadge;
use std::rc::Rc;
use utils::{events, routes::AdminUsersRoute, unwrap::UnwrapJiExt};
use web_sys::HtmlSelectElement;

impl UsersTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-table-user", {
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_users(e.query());
                }))
            }))
            .child_signal(
                state.search_count().map(|count| {
                    match count {
                        None => None,
                        Some(count) => Some(html!("p", {
                            .prop("slot", "search")
                            .text(&format!("{} users found", count))
                        }))
                    }
                })
            )
            .child(html!("table-pagination-jig", {
                .prop("slot", "controls")
                .child(html!("fa-button", {
                    .prop("slot", "back")
                    .prop("title", "Previous")
                    .prop("icon", "fa-solid fa-chevron-left")
                    .prop_signal("disabled", state.users_state.active_page.signal().map(|active_page| {
                        active_page == 0
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.users_state.active_page.get();
                        state.users_state.go_to_page(active_page - 1);
                    }))
                }))
                .child(html!("fa-button", {
                    .prop("slot", "next")
                    .prop("title", "Next")
                    .prop("icon", "fa-solid fa-chevron-right")
                    .prop_signal("disabled", map_ref! {
                        let total_pages = state.users_state.total_pages.signal(),
                        let active_page = state.users_state.active_page.signal() => {
                            match total_pages {
                                None => true,
                                Some(total_pages) => {
                                    // active_page is 0 indexed in the code side, so need to add 1 for display
                                    *active_page == total_pages - 1
                                }
                            }
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.users_state.active_page.get();
                        state.users_state.go_to_page(active_page + 1);
                    }))
                }))
                .child_signal(state.users_state.total_pages.signal().map(clone!(state => move |total_pages| {
                    total_pages.map(|total_pages| {
                        html!("input-select", {
                            .style("width", "150px")
                            .prop_signal("value", state.users_state.active_page.signal().map(|active_page| {
                                format!("{}", active_page + 1)
                            }))
                            .children((0..total_pages).map(|page| {
                                html!("input-select-option", {
                                    .text(&format!("{}", page + 1).to_string())
                                    .prop_signal("selected", state.users_state.active_page.signal().map(clone!(page => move |active_page| {
                                        page == active_page
                                    })))
                                    .event(clone!(state, page => move |evt: events::CustomSelectedChange| {
                                        if evt.selected() {
                                            state.users_state.go_to_page(page);
                                        }
                                    }))
                                })
                            }))
                        })
                    })
                })))
            }))
            .children_signal_vec(state.users_state.users.signal_vec_cloned().map(clone!(state => move |user: Rc<EditableUser>| {
                let user_id = user.id;
                html!("admin-table-line", {
                    .children(&mut [
                        html!("span", {
                            .text_signal(user.username.signal_cloned())
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminUsersRoute::User(user_id);
                                state.users_state.navigate_to(route);
                            }))
                        }),
                        html!("span", {
                            .text_signal(user.first_name.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.last_name.signal_cloned())
                        }),
                        html!("span", {
                            .child(
                                html!("a", {
                                    .prop("href", format!("mailto:{}", user.email.get_cloned()))
                                    .prop("target", "_blank")
                                    .prop("rel", "noopener norefferer")
                                    .text_signal(user.email.signal_cloned())
                                })
                            )
                        }),
                        html!("label", {
                            .child(html!("select" => HtmlSelectElement, {
                                .with_node!(select => {
                                    .prop_signal("value", user.badge.signal().map(|badge| {
                                        badge_to_json(badge)
                                    }))
                                    .children(&mut [
                                        html!("option", {
                                            .prop("value", badge_to_json(None))
                                        }),
                                        html!("option", {
                                            .text(&UserBadge::MasterTeacher.display_name())
                                            .prop("value", badge_to_json(Some(UserBadge::MasterTeacher)))
                                        }),
                                        html!("option", {
                                            .text(&UserBadge::JiTeam.display_name())
                                            .prop("value", badge_to_json(Some(UserBadge::JiTeam)))
                                        }),
                                    ])
                                    .event(clone!(state, user, select => move |_: events::Change| {
                                        let value = select.value();
                                        let value = json_to_badge(&value);
                                        user.badge.set(value);

                                        state.save_admin_data(&user);
                                    }))
                                })
                            }))
                        }),
                        html!("span", {
                            .text_signal(user.country.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.state.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.city.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.organization.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.signup_date.signal_cloned())
                        }),
                        html!("span", {
                            .text_signal(user.language.signal_cloned())
                        }),
                    ])
                })
            })))
        })
    }
}

fn badge_to_json(badge: Option<UserBadge>) -> String {
    serde_json::to_string(&badge).unwrap_ji()
}

fn json_to_badge(json: &str) -> Option<UserBadge> {
    serde_json::from_str(json).unwrap_ji()
}
