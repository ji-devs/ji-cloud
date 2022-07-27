use std::rc::Rc;

use dominator::{clone, html, DomBuilder};
use futures_signals::signal::SignalExt;
use utils::events;
use web_sys::{File, ShadowRoot, Url};

use crate::member_details::component::Component;

use super::{EditImage, ImageIfOrFile};

const STR_HEADING: &str = "Add/Edit image";

impl Component for Rc<EditImage> {
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
                                                .text("Drag & drop or browse an image")
                                            }),
                                            html!("p", {
                                                .class("file-size")
                                                .text("Maximum image size: 5 MB")
                                            }),
                                        ])
                                    })
                                },
                            })
                        }))
                )
                .child(html!("button-rect", {
                    .text("Apply")
                    .property("slot", "submit")
                    .property_signal("disabled", state.loader.is_loading())
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