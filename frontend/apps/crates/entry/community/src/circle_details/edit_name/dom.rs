use std::rc::Rc;

use super::EditName;
use dominator::{clone, html, with_node, DomBuilder};
use utils::{component::Component, events};
use web_sys::{HtmlInputElement, ShadowRoot};

pub const STR_NAME: &str = "Name";
pub const STR_NAME_PLACEHOLDER: &str = "Circle name";

impl Component<EditName> for Rc<EditName> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("popup-body", {
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }))
            .child(html!("h3", {
                .prop("slot", "heading")
                .text(STR_NAME)
            }))
            .child(html!("div", {
                .prop("slot", "body")
                .class("field-grid")
                .child(html!("div", {
                    .class("body")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("slot", "organization")
                            .child(html!("input" => HtmlInputElement, {
                                .with_node!(elem => {
                                    .prop("placeholder", STR_NAME_PLACEHOLDER)
                                    .prop_signal("value", state.display_name.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        state.display_name.set(elem.value());
                                    }))
                                })
                            }))
                        }),
                        html!("button-rect", {
                            .text("Save")
                            .prop("slot", "submit")
                            .event(clone!(state => move |_: events::Click| {
                                let circle = state.get_circle_update_data();
                                (state.callbacks.save_changes)(circle);
                            }))
                        }),
                    ])
                }))
            }))
        }))
    }
}
