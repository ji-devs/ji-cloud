use std::rc::Rc;

use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};

use utils::{component::Component, events};
use wasm_bindgen_futures::spawn_local;

use super::super::super::add_unit_value::add_file;

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
        html!("div", {
            .prop("slot", "file-input")
            .children(&mut [
                FileInput::new(FileInputConfig {
                    on_change: Box::new(clone!(state => move|file| {
                        // spawn_local(clone!(state => async move {
                            if let Some(file) = file {
                                AddFile::save(&state, file);
                            }
                        // }));
                    })),
                    error_msg_type: STR_ERROR_MSG_TYPE.to_string(),
                    error_msg_size: STR_ERROR_MSG_SIZE.to_string(),
                    accept: "image/*,audio/*,application/pdf",
                    slot: Some("input-file"),
                    ..Default::default()
                }).render(),
            ])
        })
    }
}
