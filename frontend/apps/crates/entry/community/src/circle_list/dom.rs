use std::rc::Rc;

use dominator::{class, clone, html, pseudo, with_node, DomBuilder};
use futures_signals::{map_ref, signal::SignalExt};
use utils::{component::Component, dialog, events, paywall};
use web_sys::{HtmlInputElement, ShadowRoot};

use crate::circle_card::CircleCard;

use super::{create_circle::CreateCircle, CirclesList};

impl Component<CirclesList> for Rc<CirclesList> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state: &Rc<CirclesList> = self;
        state.load_circles();
        dom.child(html!("div", {
            .class("header")
            .child(html!("div", {
                .class("left-side")
                .child(html!("h1", {
                    .text("Circles")
                }))
                .child(html!("span", {
                    .class("circles-count")
                    .text_signal(self.total_circles_count.signal().map(|t| {
                        t.map(|t| t.to_string()).unwrap_or_default()
                    }))
                }))
            }))
            .child(html!("button-rect", {
                .prop("slot", "create-button")
                .prop("color", "blue")
                .text("Start a circle")
                .event(clone!(state => move |_: events::Click| {
                    if !paywall::can_create_circle() {
                        paywall::dialog_limit("
                            Looking to create a Circle?
                            Upgrade now for UNLIMITED JIGs and resources.
                        ");
                        return;
                    }

                    state.create_popup_open.set(true);
                }))
            }))
        }))
        .children_signal_vec(
            state
                .circles
                .signal_ref(move |circles| match circles {
                    None => {
                        vec![html!("progress", {
                            .prop("slot", "items")
                        })]
                    }
                    Some(circles) => circles
                        .iter()
                        .map(|circle| CircleCard { circle }.render())
                        .collect(),
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
                            state.load_circles();
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
                                    state.load_circles();
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
                        state.load_circles();
                    }))
                }),
            ])
        }))
        .child_signal(
            state
                .create_popup_open
                .signal()
                .map(clone!(state => move |open| {
                    open.then(|| {
                        dialog!{
                            .child(CreateCircle::new(&state).render())
                        }
                    })
                })),
        )
    }
}
