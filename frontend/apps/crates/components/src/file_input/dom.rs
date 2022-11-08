use dominator::{clone, html, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{component::Component, events};
use web_sys::{File, HtmlElement, ShadowRoot, Url};

use super::FileInput;

const STR_LABEL_PRIMARY: &str = "Upload or drag file here";
const STR_LABEL_SECONDARY: &str = "Stretches to fit. Max ";

impl Component<FileInput> for Rc<FileInput> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, mut dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        if let Some(slot) = self.slot {
            dom = dom.prop("slot", slot);
        }
        dom
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom
            .child_signal(state.value.signal_ref(clone!(state => move|value| {
                value.as_ref().map(|_| {
                    html!("fa-button", {
                        .class("remove")
                        .prop("icon", "fa-regular fa-xmark")
                        .event(clone!(state => move |_: events::Click| {
                            state.delete_file();
                        }))
                    })
                })
            })))
            .child(html!("input-file", {
                .prop("accept", state.accept)
                .apply_if(state.show_border, |dom| {
                    dom.style("border", "dashed 2px var(--light-blue-4)")
                })
                .class_signal("error", state.has_error_signal())
                .event(clone!(state => move |e: events::CustomFile| {
                    state.on_file_change(e.file());
                }))
                .child_signal(state.value.signal_ref(clone!(state => move|value| {
                    match value {
                        Some(value) if state.preview_images && value.type_().starts_with("image/") => {
                            Some(html!("img", {
                                .style("overflow", "hidden")
                                .style("max-width", "100%")
                                .style("max-height", "100%")
                                .prop("src", file_to_object_url(&value))
                            }))
                        },
                        Some(value) => {
                            Some(html!("p", {
                                .text(&value.name())
                            }))
                        }
                        _ => Some(html!("div", {
                            .class("empty")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-light fa-cloud-arrow-up")
                            }))
                            .children_signal_vec(state.has_error_signal().map(clone!(state => move|has_error| {
                                match has_error {
                                    true => vec![],
                                    false => vec![
                                        html!("p", {
                                            .class("pick-file-message")
                                            .text(STR_LABEL_PRIMARY)
                                        }),
                                        html!("p", {
                                            .class("file-size")
                                            .text(STR_LABEL_SECONDARY)
                                            .text(&format!("{}", state.max_size))
                                        }),
                                    ],
                                }
                            })).to_signal_vec())
                            .child_signal(state.error_size.signal().map(clone!(state => move|error_size| {
                                error_size.then(clone!(state => move || {
                                    html!("p", {
                                        .class("error-message")
                                        .text(&state.error_msg_size)
                                    })
                                }))
                            })))
                            .child_signal(state.error_mime_type.signal().map(clone!(state => move|error_mime_type| {
                                error_mime_type.then(|| {
                                    html!("p", {
                                        .class("error-message")
                                        .text(&state.error_msg_type)
                                    })
                                })
                            })))
                        })),
                    }
                })))
            }))
    }
}

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap()
}
