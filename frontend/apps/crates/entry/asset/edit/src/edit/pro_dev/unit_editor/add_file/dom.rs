use std::rc::Rc;

use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};

use utils::{component::Component, events};

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
                        state.unit_editor_state.value.set(None);
                    }))
                }),
            ])
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
