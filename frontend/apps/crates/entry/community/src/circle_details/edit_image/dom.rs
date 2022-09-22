use std::rc::Rc;

use super::{EditImage, ImageIfOrFile};
use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, DomBuilder};
use futures_signals::{map_ref, signal::SignalExt};
use utils::{component::Component, events};
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
                                                    .property("imageId", &image_id.0.to_string())
                                                })
                                            },
                                            ImageIfOrFile::File(file) => {
                                                html!("img", {
                                                    .property("src", file_to_object_url(&file))
                                                })
                                            },
                                        })
                                        .child(html!("button-rect", {
                                            .text("Delete")
                                            .property("color", "blue")
                                            .property("kind", "text")
                                            .event(clone!(state => move |_: events::Click| {
                                                // using set_neq otherwise FileInput would be reinitialized and won't show errors
                                                state.image.set_neq(None);
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
                                        accept: "image/*",
                                        preview_images: true,
                                        ..Default::default()
                                    }).render()
                                },
                            })
                        }))
                )
                // .child_signal(
                //     state
                //         .image
                //         .signal_cloned()
                //         .map(clone!(state => move |image| {
                //             Some({
                //                 html!("div", {
                //                     .class("has-image")
                //                     .child(match image {
                //                         ImageIfOrFile::ImageId(image_id) => {
                //                             html!("profile-image", {
                //                                 .property("imageId", &image_id.0.to_string())
                //                             })
                //                         },
                //                         ImageIfOrFile::File(file) => {
                //                             html!("img", {
                //                                 .property("src", file_to_object_url(&file))
                //                             })
                //                         },
                //                     })
                //                 })
                //             })
                //         }))
                // )
                .child(html!("button-rect", {
                    .text(STR_SAVE)
                    .property("slot", "submit")
                    .property_signal("disabled", map_ref! {
                        let is_loading = state.loader.is_loading(),
                        let image = state.image.signal_cloned() => move {
                            *is_loading || image.is_none()
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        state.apply_changes();
                    }))
                }))
            }))
        }))
    }
}

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap()
}
