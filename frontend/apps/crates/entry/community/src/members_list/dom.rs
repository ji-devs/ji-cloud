use std::rc::Rc;

use dominator::{class, clone, html, pseudo, with_node, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use utils::{component::Component, events};
use web_sys::{HtmlInputElement, ShadowRoot};

use crate::member_card::MemberCard;

use super::MembersList;

impl Component<MembersList> for Rc<MembersList> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_members();

        dom.child(html!("div", {
            .class("header")
            .child(html!("h1", {
                .text("Members")
            }))
            .child(html!("span", {
                .class("member-count")
                .text_signal(self.total_user_count.signal().map(|t| {
                    t.map(|t| t.to_string()).unwrap_or_default()
                }))
            }))
        }))
        .children_signal_vec(
            state
                .members
                .signal_ref(move |members| {
                    match members {
                        None => {
                            vec![html!("progress", {
                                .prop("slot", "items")
                            })]
                        }
                        Some(members) => {
                            members
                                .iter()
                                .map(|member| {
                                    MemberCard {
                                        member,
                                        slot: "",
                                        menu: None,
                                        following: Mutable::new(false).read_only(), // TODO: use internal mutable instead
                                        on_follow: Box::new(|_| {}),
                                        admin_tag: false,
                                    }
                                    .render()
                                })
                                .collect()
                        }
                    }
                })
                .to_signal_vec(),
        )
        .child(html!("community-pagination", {
            .prop("slot", "sort-header")
            .prop_signal("total", state.total_pages.signal())
            .children(&mut [
                html!("fa-button", {
                    .prop("slot", "back")
                    .prop("icon", "fa-solid fa-angle-left")
                    .prop_signal("disabled", state.active_page.signal().map(|active_page| {
                        active_page <= 1
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.active_page.get();
                        if active_page > 1 {
                            state.active_page.set(active_page - 1);
                            state.load_members();
                        };
                    }))
                }),
                html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .class(class! {
                            .pseudo!("::-webkit-outer-spin-button", {
                                .style("-webkit-appearance", "none")
                                .style("margin", "0")
                            })
                            .pseudo!("::-webkit-inner-spin-button", {
                                .style("-webkit-appearance", "none")
                                .style("margin", "0")
                            })
                        })
                        .prop("slot", "active-page")
                        .prop("type", "number")
                        .prop("min", 1)
                        .prop_signal("max", state.total_pages.signal())
                        .prop_signal("value", state.active_page.signal().map(|active_page| {
                            active_page.to_string()
                        }))
                        .event(clone!(state, elem => move |_: events::Input| {
                            let value = elem.value();
                            if let Ok(num) = value.parse::<u32>() {
                                if num <= state.total_pages.get() {
                                    state.active_page.set(num);
                                    state.load_members();
                                }
                            };
                        }))
                    })
                }),
                html!("fa-button", {
                    .prop("slot", "forward")
                    .prop("icon", "fa-solid fa-angle-right")
                    .prop_signal("disabled", map_ref! {
                        let active_page = state.active_page.signal(),
                        let total_pages = state.total_pages.signal() => {
                            active_page >= total_pages
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        state.active_page.replace_with(|active_page| {
                            *active_page + 1
                        });
                        state.load_members();
                    }))
                }),
            ])
        }))
    }
}
