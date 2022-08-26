use std::rc::Rc;

use dominator::{clone, html, with_node, DomBuilder};
use utils::events;
use web_sys::{HtmlInputElement, ShadowRoot};

use crate::member_details::component::Component;

use super::EditName;

pub const STR_NAME: &str = "Name";
pub const STR_NAME_PLACEHOLDER: &str = "Circle name";

impl Component for Rc<EditName> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("popup-body", {
            .child(html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }))
            .child(html!("h3", {
                .property("slot", "heading")
                .text(STR_NAME)
            }))
            .child(html!("div", {
                .property("slot", "body")
                .class("field-grid")
                .child(html!("div", {
                    .class("body")
                    .children(&mut [
                        html!("input-wrapper", {
                            .property("slot", "organization")
                            .child(html!("input" => HtmlInputElement, {
                                .with_node!(elem => {
                                    .property("placeholder", STR_NAME_PLACEHOLDER)
                                    .property_signal("value", state.display_name.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        state.display_name.set(elem.value());
                                    }))
                                })
                            }))
                        }),
                        html!("button-rect", {
                            .text("Save")
                            .property("slot", "submit")
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
