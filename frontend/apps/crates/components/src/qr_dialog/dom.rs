use std::rc::Rc;

use dominator::{clone, DomBuilder, EventOptions};
use utils::{component::Component, dialog, events};
use web_sys::ShadowRoot;

use super::QrDialog;

impl Component<QrDialog> for Rc<QrDialog> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(dialog! {
            .class("qr-dialog")
            .event_with_options(&EventOptions::bubbles(), clone!(state => move |e: events::Click| {
                e.stop_propagation();
                (state.callbacks.on_close)();
            }))
            .child(html!("div", {
                .class("body")
                .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
                    e.stop_propagation();
                })
                .child(html!("fa-button", {
                    .class("close")
                    .prop("icon", "fa-regular fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        (state.callbacks.on_close)();
                    }))
                }))
                .child(html!("img", {
                    .style("max-height", "5cm")
                    .style("max-width", "5cm")
                    .prop("src", &state.url)
                }))
                .child(html!("fa-button", {
                    .class("download")
                    .prop("icon", "fa-solid fa-square-down")
                    .prop("title", "Download")
                    .event(clone!(state => move |_: events::Click| {
                        state.download();
                    }))
                }))
            }))
        })
    }
}
