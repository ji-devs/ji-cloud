use std::rc::Rc;

use dominator::{clone, html, with_node, DomBuilder};
use utils::events;
use web_sys::{HtmlTextAreaElement, ShadowRoot};

use crate::member_details::component::Component;

use super::EditBio;

const STR_HEADING: &str = "Add/Edit bio";
pub const STR_BIO: &str = "Bio";

impl Component for Rc<EditBio> {
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
                .text(STR_HEADING)
            }))
            .child(html!("div", {
                .property("slot", "body")
                .class("field-grid")
                .child(html!("div", {
                    .class("body")
                    .children(&mut [
                        html!("input-wrapper", {
                            .property("slot", "organization")
                            .property("label", STR_BIO)
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .with_node!(elem => {
                                    .property_signal("value", state.bio.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        state.bio.set(elem.value());
                                    }))
                                })
                            }))
                        }),
                        html!("community-private-public-switch", {
                            .property("type", "checkbox")
                            .property_signal("isPublic", state.bio_public.signal())
                            .event(clone!(state => move |evt: events::CustomToggle| {
                                state.bio_public.set_neq(evt.value());
                            }))
                        }),
                        html!("button-rect", {
                            .text("Apply")
                            .property("slot", "submit")
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
