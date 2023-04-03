use components::stickers::{
    embed::{ext::YoutubeUrlExt, state::Embed},
    state::Stickers,
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::module::body::_groups::design::{EmbedHost, YoutubeEmbed, YoutubeUrl};
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

const STR_DELETE: &str = "Delete";

pub fn render(state: Rc<Step2>) -> Dom {
    html!("video-third-party-input-card", {
        .prop("host", "youtube")
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .prop("slot", "input")
                .prop("label", "Add a YouTube link")
                .child(html!("input" => HtmlInputElement, {
                    .prop_signal("value", state.sidebar.base.embed.signal_cloned().map(|embed| {
                        match embed {
                            None => String::new(),
                            Some(embed) => {
                                // TODO: don't .lock_ref(), use ref_map
                                match &*embed.host.lock_ref() {
                                    EmbedHost::Youtube(youtube) => youtube.url.0.clone(),
                                }
                            },
                        }
                    }))
                    .with_node!(input => {
                        .event(clone!(state => move |_: events::Input| {
                            match YoutubeUrl::try_parse(input.value()) {
                                Err(_) => {
                                    actions::set_error(&wrapper, true);
                                }
                                Ok(youtube_url) => {
                                    actions::set_error(&wrapper, false);
                                    let host = EmbedHost::Youtube(YoutubeEmbed {
                                        url: youtube_url,
                                    });
                                    state.sidebar.base.on_link_change(host);
                                },
                            };
                        }))
                    })
                }))
            })
        }))
        .children_signal_vec(state.sidebar.base.embed.signal_ref(clone!(state => move |embed| {
            match embed {
                None => vec![],
                Some(embed) => vec![
                    html!("input-checkbox", {
                        .prop("label", "Clip video")
                        .prop("slot", "clip-checkbox")
                        .prop_signal("checked", state.sidebar.base.clip.signal())
                        // .prop_signal("disabled", map_ref! {
                        //     let start_at = embed.start_at.signal(),
                        //     let end_at = embed.end_at.signal() => {
                        //         start_at.is_some() || end_at.is_some()
                        //     }
                        // })
                        .event(clone!(state, embed => move |e: events::CustomToggle| {
                            if !e.value() {
                                // clear values if unchecked
                                embed.start_at.set(None);
                                embed.end_at.set(None);
                            }
                            state.sidebar.base.clip.set(e.value());
                        }))
                    }),
                    html!("button-rect", {
                        .prop("slot", "delete")
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .text(STR_DELETE)
                        .event(clone!(state => move |_: events::Click| {
                            state.sidebar.base.delete_embed();
                        }))
                    }),
                ],
            }
        })).to_signal_vec())
        .children_signal_vec(show_start_end_signal(&state).map(clone!(state => move |embed| {
            match embed {
                None => vec![],
                Some(embed) => vec![
                    html!("input-wrapper", {
                        .prop("slot", "start-at")
                        .prop("label", "Start time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .prop("type", "number")
                            .prop_signal("value", embed.start_at.signal_ref(|start_at| {
                                match start_at {
                                    Some(start_at) => start_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .event(clone!(state => move |e: events::CustomInputNumber| {
                                let embed = state.sidebar.base.get_embed_sticker().unwrap_ji();

                                let value = e.value().map(|num| num as u32);

                                log::info!("{value:?}");

                                embed.start_at.set(value);
                                Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                            }))
                        }))
                    }),
                    html!("input-wrapper", {
                        .prop("slot", "end-at")
                        .prop("label", "End time")
                        .child(html!("input-minutes-seconds" => HtmlElement, {
                            .prop("type", "number")
                            .prop_signal("value", embed.end_at.signal_ref(|end_at| {
                                match end_at {
                                    Some(end_at) => end_at.to_string(),
                                    None => String::new(),
                                }
                            }))
                            .event(clone!(state => move |e: events::CustomInputNumber| {
                                let embed = state.sidebar.base.get_embed_sticker().unwrap_ji();

                                let value = e.value().map(|num| num as u32);

                                log::info!("{value:?}");

                                embed.end_at.set(value);
                                Stickers::call_change(&Rc::clone(&state.sidebar.base.stickers));
                            }))
                        }))
                    }),
                ],
            }
        })).to_signal_vec())
    })
}

fn show_start_end_signal(state: &Rc<Step2>) -> impl Signal<Item = Option<Rc<Embed>>> {
    map_ref! {
        let embed = state.sidebar.base.embed.signal_cloned(),
        let clip = state.sidebar.base.clip.signal() => move {
            match embed {
                Some(embed) if *clip => Some(Rc::clone(embed)),
                _ => None,
            }
        }
    }
}
