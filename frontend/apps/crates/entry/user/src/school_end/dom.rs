use super::state::SchoolEnd;
use components::file_input::{FileInput, FileInputConfig};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::{component::Component, events};
use web_sys::HtmlInputElement;

impl SchoolEnd {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("input-wrapper", {
                .prop("label", "Organization Type")
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .prop_signal("value", state.organization_type.signal_cloned().map(|x| x.unwrap_or_default()))
                        .event(clone!(state => move |_evt: events::Input| {
                            let value = string_to_option(elem.value());
                            state.organization_type.set(value);
                        }))
                    })
                }))
            }))
            .child(html!("input-wrapper", {
                .prop("label", "Website")
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .prop("type", "url")
                        .prop_signal("value", state.website.signal_cloned().map(|x| x.unwrap_or_default()))
                        .event(clone!(state => move |_evt: events::Input| {
                            let value = string_to_option(elem.value());
                            state.website.set(value);
                        }))
                    })
                }))
            }))
            .child(html!("input-wrapper", {
                .prop("label", "Description")
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .prop_signal("value", state.description.signal_cloned().map(|x| x.unwrap_or_default()))
                        .event(clone!(state => move |_evt: events::Input| {
                            let value = string_to_option(elem.value());
                            state.description.set(value);
                        }))
                    })
                }))
            }))
            .child(
                FileInput::new(FileInputConfig {
                    on_change: Box::new(move |file| {
                        if let Some(_file) = file {
                            todo!();
                            // state.file.set(Some(file));
                            //     state.save();
                            // state.profile_image.set_neq(Some(file));
                        }
                    }),
                    preview_images: true,
                    accept: "image/*",
                    ..Default::default()
                }).render()
            )
            .child(html!("button-rect", {
                .text("Next")
                .event(clone!(state => move |_: events::Click| {
                    state.save();
                }))
            }))
        })
    }
}

// convert empty strings into None
fn string_to_option(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}
