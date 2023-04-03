use std::rc::Rc;

use super::EditAbout;
use dominator::{clone, html, with_node, DomBuilder};
use utils::{component::Component, events};
use web_sys::{HtmlTextAreaElement, ShadowRoot};

pub const STR_ABOUT: &str = "About";
pub const STR_ABOUT_PLACEHOLDER: &str = "";

impl Component<EditAbout> for Rc<EditAbout> {
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
                .text(STR_ABOUT)
            }))
            .child(html!("div", {
                .prop("slot", "body")
                .class("field-grid")
                .child(html!("div", {
                    .class("body")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("slot", "organization")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .prop("placeholder", STR_ABOUT_PLACEHOLDER)
                                .prop("dir", "auto")
                                .with_node!(elem => {
                                    .prop_signal("value", state.description.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        state.description.set(elem.value());
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
