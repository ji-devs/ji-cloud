use std::rc::Rc;

use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, Dom};

use futures_signals::signal::SignalExt;
use utils::component::Component;

use super::state::AddFile;

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
                        if let Some(file) = file {
                            state.file.set(Some(file));
                            state.save();
                        }
                    })),
                    error_msg_type: STR_ERROR_MSG_TYPE.to_string(),
                    error_msg_size: STR_ERROR_MSG_SIZE.to_string(),
                    accept: "image/*,audio/*,application/pdf",
                    slot: Some("input-file"),
                    ..Default::default()
                }).render(),
            ])
            .child_signal(state.loader.is_loading().map(move|is_loading| {
                match is_loading {
                    true => Some(
                        html!("div", {
                            .children(&mut [
                                html!("progress-bar", {
                                    .prop("progress", "infinite")
                                })
                            ])
                        })
                    ),
                    false => Some(
                        html!("div", {

                        })
                    )
                }
            }))
        })
    }
}
