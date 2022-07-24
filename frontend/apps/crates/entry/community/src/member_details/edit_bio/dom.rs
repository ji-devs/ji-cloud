use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use utils::events;
use web_sys::HtmlTextAreaElement;

use super::EditBio;

pub const STR_BIO: &str = "Bio";

impl EditBio {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .style("background", "white")
            .style("padding", "30px")
            .style("border-radius", "16px")
            .style("box-shadow", "0 3px 6px 0 rgba(0, 0, 0, 0.16)")
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
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),

                html!("input-checkbox", {
                    .property("type", "checkbox")
                    .property_signal("checked", state.bio_public.signal())
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
                html!("fa-button", {
                    .property("slot", "close")
                    .property("icon", "fa-regular fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        (state.callbacks.close)();
                    }))
                }),
            ])
        })
    }
}
