use std::{cell::RefCell, rc::Rc};

use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use utils::{component::Component, events, prelude::get_user_id};
use web_sys::{HtmlElement, ScrollIntoViewOptions};

use crate::{circle_card::CircleCard, member_card::MemberCard};

use super::CommunitySearch;

const STR_WE_FOUND: &str = "We found";
const STR_AND: &str = "and";
const STR_FOR: &str = "for";
const STR_MEMBERS: &str = "Members";
const STR_CIRCLES: &str = "Circles";
const STR_SEE_MORE: &str = "See more";

impl Component<CommunitySearch> for Rc<CommunitySearch> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(
        &self,
        dom: dominator::DomBuilder<web_sys::ShadowRoot>,
    ) -> dominator::DomBuilder<web_sys::ShadowRoot> {
        let state = self;
        state.search();

        let current_user_id = get_user_id();

        dom.child(html!("h2", {
            .text(STR_WE_FOUND)
            .text(" ")
            .child(html!("a", {
                .text_signal(state.member_count.signal().map(|c| c.to_string()))
                .text(" ")
                .text(STR_MEMBERS)
                .event(clone!(state => move |_: events::Click| {
                    state.scroll_into_view(&state.members_el);
                }))
            }))
            .text(" ")
            .text(STR_AND)
            .text(" ")
            .child(html!("a", {
                .text_signal(state.circle_count.signal().map(|c| c.to_string()))
                .text(" ")
                .text(STR_CIRCLES)
                .event(clone!(state => move |_: events::Click| {
                    state.scroll_into_view(&state.circles_el);
                }))
            }))
            .text(" ")
            .text(STR_FOR)
            .prop("query", &state.query.q)
        }))
        .child(html!("section", {
            .after_inserted(clone!(state => move |el: HtmlElement| {
                *state.members_el.borrow_mut() = Some(el);
            }))
            .child(html!("h4", {
                .text(STR_MEMBERS)
                .text(" (")
                .text_signal(state.member_count.signal().map(|c| c.to_string()))
                .text(")")
            }))
            .children_signal_vec(state.members.signal_vec_cloned().map(move |member| {
                MemberCard {
                    member: &member,
                    admin_tag: false,
                    menu: None,
                    current_user_id,
                }
                .render()
            }))
            .child(html!("div", {
                .class("see-more-wrapper")
                .child_signal(state.render_see_more_members())
            }))
        }))
        .child(html!("section", {
            .after_inserted(clone!(state => move |el: HtmlElement| {
                *state.circles_el.borrow_mut() = Some(el);
            }))
            .child(html!("h4", {
                .text(STR_CIRCLES)
                .text(" (")
                .text_signal(state.circle_count.signal().map(|c| c.to_string()))
                .text(")")
            }))
            .children_signal_vec(state.circles.signal_vec_cloned().map(move |circle| {
                CircleCard {
                    circle: &circle,
                }.render()
            }))
            .child(html!("div", {
                .class("see-more-wrapper")
                .child_signal(state.render_see_more_circles())
            }))
        }))
        // }))
    }
}

impl CommunitySearch {
    fn render_see_more_members(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = Rc::clone(self);
        map_ref! {
            let member_count = state.member_count.signal(),
            let member_len = state.members.signal_vec_cloned().len() => move {
                if *member_count > *member_len as u32 {
                    Some(html!("button-rect", {
                        .prop("slot", "members-see-more")
                        .prop("color", "blue")
                        .prop_signal("disabled", state.loader.is_loading())
                        .text(STR_SEE_MORE)
                        .event(clone!(state => move |_: events::Click| {
                            state.load_more_members();
                        }))
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn render_see_more_circles(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = Rc::clone(self);
        map_ref! {
            let circle_count = state.circle_count.signal(),
            let circle_len = state.circles.signal_vec_cloned().len() => move {
                if *circle_count > *circle_len as u32 {
                    Some(html!("button-rect", {
                        .prop("slot", "circles-see-more")
                        .prop("color", "blue")
                        .prop_signal("disabled", state.loader.is_loading())
                        .text(STR_SEE_MORE)
                        .event(clone!(state => move |_: events::Click| {
                            state.load_more_circles();
                        }))
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn scroll_into_view(&self, el: &RefCell<Option<HtmlElement>>) {
        if let Some(el) = &*el.borrow() {
            el.scroll_into_view_with_scroll_into_view_options(
                &ScrollIntoViewOptions::new().behavior(web_sys::ScrollBehavior::Smooth),
            );
        }
    }
}
