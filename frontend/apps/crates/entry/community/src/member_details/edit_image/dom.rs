use std::rc::Rc;

use super::{EditImage, ImageIfOrFile};
use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, DomBuilder};
use futures_signals::signal::SignalExt;
use utils::{component::Component, events, unwrap::UnwrapJiExt};
use web_sys::{File, ShadowRoot, Url};

const STR_HEADING: &str = "Profile picture";
const STR_SAVE: &str = "Save";

impl Component<EditImage> for Rc<EditImage> {
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
                .text(STR_HEADING)
            }))
            .child(html!("div", {
                .prop("slot", "body")
                .class("body")
                .class("field-grid")
                .child_signal(
                    state
                        .image
                        .signal_cloned()
                        .map(clone!(state => move |image| {
                            Some(match image {
                                Some(image) => {
                                    html!("div", {
                                        .class("has-image")
                                        .child(match image {
                                            ImageIfOrFile::ImageId(image_id) => {
                                                html!("profile-image", {
                                                    .prop("imageId", &image_id.0.to_string())
                                                })
                                            },
                                            ImageIfOrFile::File(file) => {
                                                html!("img", {
                                                    .prop("src", file_to_object_url(&file))
                                                })
                                            },
                                        })
                                        .child(html!("button-rect", {
                                            .text("Delete")
                                            .prop("color", "blue")
                                            .prop("kind", "text")
                                            .event(clone!(state => move |_: events::Click| {
                                                state.image.set(None);
                                            }))
                                        }))
                                    })
                                },
                                None => {
                                    FileInput::new(FileInputConfig {
                                        on_change: Box::new(clone!(state => move |file| {
                                            let file = file.map(|file| ImageIfOrFile::File(file));
                                            state.image.set_neq(file);
                                        })),
                                        preview_images: true,
                                        accept: "image/*",
                                        ..Default::default()
                                    }).render()
                                },
                            })
                        }))
                )
                .child(html!("button-rect", {
                    .text(STR_SAVE)
                    .prop("slot", "submit")
                    .prop_signal("disabled", state.loader.is_loading())
                    .event(clone!(state => move |_: events::Click| {
                        state.apply_changes();
                    }))
                }))
            }))
        }))
    }
}

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap_ji()
}
