
use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    user::public_user::PublicUser,
};
use std::rc::Rc;
use utils::{events, routes::AdminUsersRoute};

impl UsersTable {
    pub fn render(self: Rc<Self>) -> Dom {

        let state = self;
        html!("admin-table", {
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_users(e.query());
                }))
            }))
            .child(html!("table-pagination", {
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
            .children_signal_vec(state.users_state.users.signal_vec_cloned().map(clone!(state => move |user: Rc<PublicUser>| {
                let user_id = user.id;
                html!("admin-table-line", {
                    .child(html!("div", {
                        .style("display", "grid")
                        .style("grid-template-columns", "repeat(3, 100px)")
                        .style("align-items", "start")
                        .style("padding", "0")
                    }))
                    .children(&mut [
                        html!("a", {
                            .text(&user.username)
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminUsersRoute::User(user_id);
                                state.users_state.navigate_to(route);
                            }))
                        }),
                        html!("span", {
                            .text(&user.given_name)
                        }),
                        html!("span", {
                            .text(&user.family_name)
                        }),
                    ])
                })
            })))
        })
    }
}
