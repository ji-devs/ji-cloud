use crate::stickers::embed::types::{ParseUrlExt, PartialYoutubeEmbed};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::YoutubeUrl;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_youtube_input(
        self: &Rc<Self>,
        youtube: &Rc<PartialYoutubeEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match youtube.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, youtube => move |_: events::Input| {
                    match YoutubeUrl::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            youtube.url.set(None);
                        }
                        Ok(youtube_url) => {
                            actions::set_error(&wrapper, false);
                            youtube.url.set(Some(youtube_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }

    pub fn render_youtube_specific_options(
        self: &Rc<Self>,
        youtube: &Rc<PartialYoutubeEmbed>,
    ) -> Dom {
        let state = self;
        html!("div", {
            .child_signal(youtube.url.signal_ref(clone!(state, youtube => move |youtube_url| {
                youtube_url.as_ref().map(|_| {
                    html!("input-checkbox", {
                        .prop("label", "Clip video")
                        .prop("slot", "clip-checkbox")
                        .prop_signal("checked", youtube.clip.signal())
                        // .prop_signal("disabled", map_ref! {
                        //     let start_at = embed.start_at.signal(),
                        //     let end_at = embed.end_at.signal() => {
                        //         start_at.is_some() || end_at.is_some()
                        //     }
                        // })
                        .event(clone!(state, youtube => move |e: events::CustomToggle| {
                            if !e.value() {
                                // clear values if unchecked
                                youtube.start_at.set(None);
                                youtube.end_at.set(None);
                                state.on_embed_value_change();
                            }
                            youtube.clip.set(e.value());
                        }))
                    })
                })
            })))
            .children_signal_vec(youtube.clip.signal().map(clone!(state, youtube => move |clip| {
                match clip {
                    false => vec![],
                    true => vec![
                        html!("input-wrapper", {
                            .prop("slot", "start-at")
                            .prop("label", "Start time")
                            .child(html!("input-minutes-seconds" => HtmlElement, {
                                .prop("type", "number")
                                .prop_signal("value", youtube.start_at.signal_ref(|start_at| {
                                    match start_at {
                                        Some(start_at) => start_at.to_string(),
                                        None => String::new(),
                                    }
                                }))
                                .event(clone!(state, youtube => move |e: events::CustomInputNumber| {
                                    let value = e.value().map(|num| num as u32);
                                    youtube.start_at.set(value);
                                    state.on_embed_value_change();
                                }))
                            }))
                        }),
                        html!("input-wrapper", {
                            .prop("slot", "end-at")
                            .prop("label", "End time")
                            .child(html!("input-minutes-seconds" => HtmlElement, {
                                .prop("type", "number")
                                .prop_signal("value", youtube.end_at.signal_ref(|end_at| {
                                    match end_at {
                                        Some(end_at) => end_at.to_string(),
                                        None => String::new(),
                                    }
                                }))
                                .event(clone!(state, youtube => move |e: events::CustomInputNumber| {
                                    let value = e.value().map(|num| num as u32);
                                    youtube.end_at.set(value);
                                    state.on_embed_value_change();
                                }))
                            }))
                        }),
                    ],
                }
            })).to_signal_vec())
        })
    }
}
