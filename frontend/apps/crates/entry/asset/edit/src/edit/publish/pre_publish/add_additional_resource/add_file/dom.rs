use std::rc::Rc;

use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};

use utils::{component::Component, events};

use super::super::super::add_additional_resource::ActivePopup;

use super::state::AddFile;

const STR_SAVE: &str = "Save";
const STR_CANCEL: &str = "Cancel";
const STR_BACK: &str = "Back";
const STR_ERROR_MSG_TYPE: &str =
    "Oh no! We don't accept that type of file. We accept all image, audio and PDF files.";
const STR_ERROR_MSG_SIZE: &str = "Oh no! This file is too heavy. Maximum file size: 5 MB.";

impl AddFile {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(self);
        html!("jig-edit-publish-resource-add-file", {
            .children(&mut [
                html!("button-rect", {
                    .prop("slot", "back")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-solid fa-angle-left")
                    }))
                    .text(STR_BACK)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(Some(ActivePopup::Main));
                    }))
                }),
                html!("fa-button", {
                    .prop("icon", "fa-light fa-xmark")
                    .prop("slot", "close")
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                FileInput::new(FileInputConfig {
                    on_change: Box::new(clone!(state => move|file| {
                        state.file.set(file);
                    })),
                    error_msg_type: STR_ERROR_MSG_TYPE.to_string(),
                    error_msg_size: STR_ERROR_MSG_SIZE.to_string(),
                    accept: "image/*,audio/*,application/pdf",
                    slot: Some("input-file"),
                    ..Default::default()
                }).render(),
                html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text(STR_CANCEL)
                    .event(clone!(state => move|_: events::Click| {
                        state.add_resources_state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "actions")
                    .prop("color", "blue")
                    .prop("kind", "filled")
                    .text(STR_SAVE)
                    .prop_signal("disabled", not(state.form_filled_out()))
                    .event(clone!(state => move|_: events::Click| {
                        state.save();
                    }))
                }),
            ])
            .child_signal(state.add_resources_state.publish_state.resource_types.signal_ref(clone!(state => move|resource_types| {
                Some(html!("input-select", {
                    .prop("slot", "type")
                    .prop("label", "Select type")
                    .prop("placeholder", "Select one")
                    .prop_signal("value", state.resource_type.signal_ref(|resource_type| {
                        match resource_type {
                            Some(resource_type) => resource_type.display_name.clone(),
                            None => String::new(),
                        }
                    }))
                    .children(resource_types.iter().map(clone!(state => move |resource_type| {
                        html!("input-select-option", {
                            .text(&resource_type.display_name)
                            .event(clone!(state, resource_type => move |evt:events::CustomSelectedChange| {
                                if evt.selected() {
                                    state.resource_type.set(Some(resource_type.clone()));
                                }
                            }))
                        })
                    })))
                }))
            })))
        })
    }

    fn form_filled_out(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let file = self.file.signal_cloned(),
            let resource_type = self.resource_type.signal_cloned()
                => move {
                    file.is_some() && resource_type.is_some()
                }
        }
    }
}
