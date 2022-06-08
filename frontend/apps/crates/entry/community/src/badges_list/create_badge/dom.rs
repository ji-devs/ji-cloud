use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use utils::events;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use super::CreateBadge;

pub const STR_BADGE_NAME: &str = "Your badge name";
pub const STR_DESCRIPTION: &str = "Description";

impl CreateBadge {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("community-create-badge", {
            .property("slot", "popup")
            .children(&mut [
                html!("input-wrapper", {
                    .property("label", STR_BADGE_NAME)
                    .property("slot", "name")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                let value = input.value();
                                *state.name.borrow_mut() = value;
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .property("label", STR_DESCRIPTION)
                    .property("slot", "description")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                let value = input.value();
                                *state.description.borrow_mut() = value;
                            }))
                        })
                    }))
                }),
                html!("button-rect", {
                    .text("Create")
                    .property("slot", "submit")
                    .property_signal("disabled", state.loader.is_loading())
                    .event(clone!(state => move |_: events::Click| {
                        state.save_badges();
                    }))
                }),
                html!("fa-button", {
                    .property("slot", "close")
                    .property("icon", "fa-regular fa-xmark")
                    .property_signal("disabled", state.loader.is_loading())
                    .event(clone!(state => move |_: events::Click| {
                        state.badge_list_state.create_popup_open.set(false);
                    }))
                }),
            ])
        })
    }
}
