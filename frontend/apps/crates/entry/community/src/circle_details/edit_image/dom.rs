use std::rc::Rc;

use super::{EditImage, ImageIfOrFile};
use dominator::{clone, html, DomBuilder};
use futures_signals::{map_ref, signal::SignalExt};
use utils::{component::Component, events};
use web_sys::{File, ShadowRoot, Url};

const STR_HEADING: &str = "Profile picture";
const STR_LABEL_PRIMARY: &str = "Upload or drag image here";
const STR_LABEL_SECONDARY: &str = "Stretches to fit. Max 5 MB";
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
                                                state.image.set(None);
                                            }))
                                        }))
                                    })
                                },
                                None => {
                                    html!("input-file", {
                                        .event(clone!(state => move |evt: events::CustomFile| {
                                            let file = evt.file();
                                            state.image.set(Some(ImageIfOrFile::File(file)))
                                        }))
                                        .children(&mut [
                                            html!("fa-icon", {
                                                .property("icon", "fa-light fa-cloud-arrow-up")
                                            }),
                                            html!("p", {
                                                .class("pick-file-message")
                                                .text(STR_LABEL_PRIMARY)
                                            }),
                                            html!("p", {
                                                .class("file-size")
                                                .text(STR_LABEL_SECONDARY)
                                            }),
                                        ])
                                    })
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
