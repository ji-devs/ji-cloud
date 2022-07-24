use std::rc::Rc;

use dominator::{clone, html, DomBuilder};
use futures_signals::signal::SignalExt;
use utils::events;
use web_sys::{File, ShadowRoot, Url};

use crate::member_details::component::Component;

use super::{EditImage, ImageIfOrFile};

impl Component for Rc<EditImage> {
    fn styles() -> &'static str {
        r#"
            :host {
                background: white;
                padding: 30px;
                border-radius: 16px;
                box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
            }
        "#
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child_signal(
            state
                .image
                .signal_cloned()
                .map(clone!(state => move |image| {
                    Some(match image {
                        Some(image) => {
                            html!("div", {
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
                                .child(html!("delete-rect", {
                                    .text("Delete")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.image.set(None);
                                    }))
                                }))
                            })
                        },
                        None => {
                            // html!("fa-icon", {
                            //     .property("icon", "fa-thin fa-user-tie-hair")
                            // })
                            html!("input-file", {
                                .text("upload")
                                .event(clone!(state => move |evt: events::CustomFile| {
                                    let file = evt.file();
                                    state.image.set(Some(ImageIfOrFile::File(file)))
                                }))
                            })
                        },
                    })
                })),
        )
        .children(&mut [
            html!("button-rect", {
                .text("Apply")
                .property("slot", "submit")
                .event(clone!(state => move |_: events::Click| {
                    state.apply_changes();
                }))
            }),
            html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-regular fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }),
        ])
    }
}

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap()
}
