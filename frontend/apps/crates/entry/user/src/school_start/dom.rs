use super::state::SchoolStart;
use dominator::{clone, html, with_node, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{component::Component, events, unwrap::UnwrapJiExt};
use web_sys::{HtmlInputElement, ShadowRoot};

impl Component<SchoolStart> for Rc<SchoolStart> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("auth-page", {
            .prop("img", "entry/user/side/main.webp")
            .child(html!("main", {
                .child(html!("h1", {
                    .text("Start a school plan")
                }))
                // html!("p", {
                //     .text("Plan: ")
                //     .text(state.plan_type.as_str())
                // })
                .child(html!("h2", {
                    .text("Try Jigzi School FREE for 14 days")
                    .text(" : ")
                    .text(state.plan_type.as_str())
                }))
                .child(html!("hr"))
                .child(html!("h2", {
                    .text("Who is this plan for?")
                }))
                .child(html!("div", {
                    .class("inputs")
                    .child(html!("input-wrapper", {
                        .prop("label", "Name")
                        .child(html!("input" => HtmlInputElement, {
                            .with_node!(elem => {
                                .prop_signal("value", state.name.signal_cloned())
                                .event(clone!(state => move |_evt: events::Input| {
                                    let value = elem.value();
                                    state.name.set(value);
                                }))
                            })
                        }))
                    }))
                    .child(html!("input-wrapper", {
                        .prop("label", "Location")
                        .child(html!("input-location", {
                            .prop_signal("locationAsString", state.location.signal_cloned().map(|location| {
                                location.unwrap_or_default()
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_owned()
                            }))
                            .event(clone!(state => move |evt: events::GoogleLocation| {
                                let raw = serde_json::to_value(evt.raw_json()).unwrap_ji();
                                state.location.set(Some(raw));
                            }))
                        }))
                    }))
                }))
                .child(html!("button-rect", {
                    .text("Continue")
                    .event(clone!(state => move |_: events::Click| {
                        state.save();
                    }))
                }))
            }))
        }))
    }
}
