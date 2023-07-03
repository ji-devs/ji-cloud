use super::state::SchoolStart;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

impl SchoolStart {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
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
            .child(html!("button-rect", {
                .text("Next")
                .event(clone!(state => move |_: events::Click| {
                    state.save();
                }))
            }))
        })
    }
}
