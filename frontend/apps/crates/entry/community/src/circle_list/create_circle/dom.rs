use std::rc::Rc;

use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::not;
use utils::{component::Component, events};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use super::CreateCircle;

pub const STR_CIRCLE_NAME: &str = "Circle name";
pub const STR_DESCRIPTION: &str = "Description";

impl CreateCircle {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("community-create-circle", {
            .prop("slot", "popup")
            .children(&mut [
                html!("input-wrapper", {
                    .prop("label", STR_CIRCLE_NAME)
                    .prop("slot", "name")
                    .child(html!("input" => HtmlInputElement, {
                        .prop("placeholder", "Type a name")
                        .prop("dir", "auto")
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                let value = input.value();
                                state.name.set(Some(value));
                            }))
                        })
                    }))
                }),
                html!("input-wrapper", {
                    .prop("label", STR_DESCRIPTION)
                    .prop("slot", "description")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .prop("placeholder", "Describe why members would want to join this circle")
                        .prop("dir", "auto")
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                let value = input.value();
                                state.description.set(Some(value));
                            }))
                        })
                    }))
                }),
                html!("button-rect", {
                    .text("Create")
                    .prop("slot", "submit")
                    .prop_signal("disabled", not(state.can_save_signal()))
                    .event(clone!(state => move |_: events::Click| {
                        state.save_circles();
                    }))
                }),
                html!("fa-button", {
                    .prop("slot", "close")
                    .prop("icon", "fa-regular fa-xmark")
                    .prop_signal("disabled", state.loader.is_loading())
                    .event(clone!(state => move |_: events::Click| {
                        state.circle_list_state.create_popup_open.set(false);
                    }))
                }),
            ])
            .child(
                FileInput::new(FileInputConfig {
                    on_change: Box::new(clone!(state => move |file| {
                        state.image.set_neq(file);
                    })),
                    accept: "image/*",
                    slot: Some("image"),
                    preview_images: true,
                    ..Default::default()
                }).render()
            )
        })
    }
}
