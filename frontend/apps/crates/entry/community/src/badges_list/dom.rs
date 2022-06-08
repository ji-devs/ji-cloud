use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::badge::Badge;
use utils::events;

use super::{create_badge::CreateBadge, BadgesList};

impl BadgesList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_badges();

        html!("community-list", {
            .property("header", "badges")
            .child(html!("button-rect", {
                .property("slot", "create-button")
                .property("color", "blue")
                .text("+ badge")
                .event(clone!(state => move |_: events::Click| {
                    state.create_popup_open.set(true);
                }))
            }))
            .child(html!("community-list-badge-header", {
                .property("slot", "sort-header")
            }))
            .children_signal_vec(state.badges.signal_vec_cloned().map(clone!(state => move |badge| {
                state.render_badge(badge)
            })))
            .child_signal(state.create_popup_open.signal().map(clone!(state => move |open| {
                match open {
                    false => None,
                    true => {
                        Some(html!("empty-fragment", {
                            .style("display", "none")
                            .apply(OverlayHandle::lifecycle(clone!(state => move || {
                                CreateBadge::new(Rc::clone(&state)).render()
                            })))
                        }))
                    },
                }
            })))
        })
    }

    fn render_badge(self: &Rc<Self>, badge: Badge) -> Dom {
        html!("community-list-badge", {
            .property("slot", "items")
            .property("name", &badge.display_name)
            .property("member-count", badge.member_count)
            .property("description", &badge.description)
            .child(html!("img", {
                .property("slot", "img")
                .property("src", badge.thumbnail.as_str())
            }))
            .child(html!("community-list-badge-status", {
                .property("slot", "status")
                .property("status", "")
            }))
        })
    }
}
