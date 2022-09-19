use std::rc::Rc;

use dominator::{clone, html, DomBuilder};
use futures_signals::signal::SignalExt;
use utils::{component::Component, events};
use web_sys::{HtmlElement, ShadowRoot};

use super::FileInput;

const STR_LABEL_PRIMARY: &str = "Upload or drag file here";
const STR_LABEL_SECONDARY: &str = "Stretches to fit. Max ";
const STR_FILE_TOO_LARGE: &str = "The file you selected is too large. Max size is ";
const STR_WRONG_FILE_TYPE: &str = "The file you selected is not of a type we that accept";

impl Component<FileInput> for Rc<FileInput> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, mut dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        if let Some(slot) = self.slot {
            dom = dom.property("slot", slot);
        }
        dom
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("input-file", {
            .property("accept", state.accept)
            .apply_if(state.show_border, |dom| {
                dom.style("border", "dashed 2px var(--light-blue-4)")
            })
            .class_signal("error", state.has_error_signal())
            .event(clone!(state => move |e: events::CustomFile| {
                state.on_file_change(e.file());
            }))
            .child(html!("fa-icon", {
                .property("icon", "fa-light fa-cloud-arrow-up")
            }))
            .children_signal_vec(state.has_error_signal().map(clone!(state => move|error_size| {
                match error_size {
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
                        .text(STR_FILE_TOO_LARGE)
                        .text(&format!("{}", state.max_size))
                    })
                }))
            })))
            .child_signal(state.error_mime_type.signal().map(|error_size| {
                error_size.then(|| {
                    html!("p", {
                        .class("error-message")
                        .text(STR_WRONG_FILE_TYPE)
                    })
                })
            }))
        }))
    }
}
