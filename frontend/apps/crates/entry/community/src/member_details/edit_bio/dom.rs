use std::rc::Rc;

use super::EditBio;
use dominator::{clone, html, with_node, DomBuilder};
use utils::{component::Component, events};
use web_sys::{HtmlTextAreaElement, ShadowRoot};

pub const STR_BIO: &str = "Bio";
pub const STR_MY_BIO: &str = "My bio";
pub const STR_BIO_PLACEHOLDER: &str = "Tell the community about yourself";

impl Component<EditBio> for Rc<EditBio> {
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
                .text(STR_BIO)
            }))
            .child(html!("div", {
                .prop("slot", "body")
                .class("field-grid")
                .child(html!("div", {
                    .class("body")
                    .children(&mut [
                        html!("input-wrapper", {
                            .prop("slot", "organization")
                            .prop("label", STR_MY_BIO)
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .prop("placeholder", STR_BIO_PLACEHOLDER)
                                .with_node!(elem => {
                                    .prop_signal("value", state.bio.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        state.bio.set(elem.value());
                                    }))
                                })
                            }))
                        }),
                        html!("community-private-public-switch", {
                            .prop("type", "checkbox")
                            .prop_signal("isPublic", state.bio_public.signal())
                            .event(clone!(state => move |evt: events::CustomToggle| {
                                state.bio_public.set_neq(evt.value());
                            }))
                        }),
                        html!("button-rect", {
                            .text("Save")
                            .prop("slot", "submit")
                            .event(clone!(state => move |_: events::Click| {
                                let user = state.get_user_profile_from_fields();
                                (state.callbacks.save_changes)(user);
                            }))
                        }),
                    ])
                }))
            }))
        }))
    }
}
