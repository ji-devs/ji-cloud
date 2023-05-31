use std::rc::Rc;

use super::{ImageIdOrFile, ProfileImage};
use crate::{
    dialog::Dialog,
    file_input::{FileInput, FileInputConfig},
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use utils::{component::Component, events, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;
use web_sys::{File, HtmlElement, ShadowRoot, Url};

const STR_HEADING: &str = "Profile picture";
const STR_SAVE: &str = "Save";

impl Component<ProfileImage> for Rc<ProfileImage> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        dom.class("profile-image").prop("slot", "profile-image")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("profile-image", {
                .class("image-slot")
                .prop("slot", "profile-image")
                .prop_signal("imageId", state.profile_image.signal_ref(|profile_image| {
                    match profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
            }))
        }))
        .child(html!("fa-button", {
            .class("edit-button")
            .prop("slot", "edit-profile-image")
            .prop("icon", "fa-light fa-pen")
            .event(clone!(state => move |_: events::Click| {
                state.popup_open.set(true);
            }))
            .child_signal(state.popup_open.signal().map(clone!(state => move |popup_open| {
                    if popup_open {
                        Some(Dialog::render(
                            clone!(state => move || {
                                render_popup(&state)}),
                                Some(Box::new(clone!(state => move || {
                                    state.popup_open.set(false);
                                })))
                        ))
                    } else {
                        None
                    }
                }))
            )
        }))
    }
}

fn render_popup(state: &Rc<ProfileImage>) -> Dom {
    html!("popup-image", {
        .child(html!("style", {
            .text(include_str!("./styles.css"))
        }))
        .child(html!("popup-body", {
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    state.popup_open.set(false);
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
                                            ImageIdOrFile::ImageId(image_id) => {
                                                html!("profile-image", {
                                                    .prop("imageId", &image_id.0.to_string())
                                                })
                                            },
                                            ImageIdOrFile::File(file) => {
                                                log::info!("No Image");
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
                                            let file = file.map(|file| ImageIdOrFile::File(file));
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
    })
}

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap_ji()
}
