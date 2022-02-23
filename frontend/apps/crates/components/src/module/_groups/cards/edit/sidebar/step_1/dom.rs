use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt, map_ref};
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;
use utils::prelude::*;

use crate::{
    image::search::dom::render as render_image_search,
    lists::{dual::dom::render as render_dual_list, single::dom::render as render_single_list},
    module::_groups::cards::{edit::{state::*, strings}, lookup::Side},
    tabs::{MenuTab, MenuTabKind}, audio::input::{AudioInput, AudioInputOptions, AudioInputCallbacks},
};

const STR_NONEMPTY_LIST_LABEL: &str = "Edit your words on the cards";
const STR_EMPTY_AUDIO_SELECTION: &str = "Select a card or a pair of cards to add audio";

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("empty-fragment", {
        .style("display", "contents")
        .child_signal(state.base.is_empty_signal().map(clone!(state => move |is_empty| {
            Some(html!("menu-tabs", {
                .children(state.tabs.get().unwrap_ji().iter().enumerate().map(|(idx, tab)| {
                    let enabled = idx == 0 || (idx > 0 && !is_empty);
                    render_tab(state.clone(), tab.kind(), idx, enabled)
                }))
                .child(html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child_signal(state.tab_index.signal_cloned().map(clone!(state, is_empty => move |current_tab_idx| {
                            let tab = match current_tab_idx {
                                Some(current_tab_idx) => match state.tabs.get() {
                                    Some(tabs) => tabs.get(current_tab_idx),
                                    None => None,
                                },
                                None => None,
                            };

                            match tab {
                                Some(Tab::Single(single)) => {
                                    if !is_empty {
                                        Some(render_non_empty(state.clone()))
                                    } else {
                                        Some(render_single_list(single.clone()))
                                    }
                                },
                                Some(Tab::Dual(dual)) => {
                                    if !is_empty {
                                        Some(render_non_empty(state.clone()))
                                    } else {
                                        Some(render_dual_list(dual.clone()))
                                    }
                                }
                                Some(Tab::Image(image)) => {
                                    Some(render_image_search(image.clone(), None))
                                },
                                Some(Tab::Audio) => {
                                    let audio_signal = |state: Rc<Step1<RawData, E>>|  map_ref! {
                                        let selected_pair = state.base.selected_pair.signal_cloned(),
                                        let pairs = state.base.pairs.signal_vec_cloned().to_signal_cloned()
                                            => {
                                                match selected_pair {
                                                    Some((idx, side)) => {
                                                        let pair = pairs.get(*idx).unwrap_ji();
                                                        let audio = match side {
                                                            SelectedSide::One(side) => {
                                                                let card = match side {
                                                                    Side::Left => &pair.0,
                                                                    Side::Right => &pair.1,
                                                                };

                                                                card.audio.clone()
                                                            },
                                                            SelectedSide::Both => {
                                                                // Only use the audio if both pairs have the _same_ audio file
                                                                if pair.0.audio.is_some()
                                                                    && pair.1.audio.is_some()
                                                                    && pair.0.audio == pair.1.audio
                                                                {
                                                                    pair.0.audio.clone()
                                                                } else {
                                                                    None
                                                                }
                                                            },
                                                        };

                                                        (Some(idx.clone()), audio, Some(side.clone()))
                                                    },
                                                    None => (None, None, None)
                                                }
                                            }
                                    };

                                    Some(html!("empty-fragment", {
                                        .child_signal(audio_signal(state.clone()).map(clone!(state => move |(idx, _, selected_side)| {
                                            if let Some(idx) = idx {

                                                let opts = AudioInputOptions::new(Some(
                                                    audio_signal(state.clone()).map(|(_, audio, _)| audio),
                                                ));

                                                let callbacks = AudioInputCallbacks::new(
                                                    Some(clone!(state, selected_side => move |audio: Audio| {
                                                        state.base.replace_pair(idx, |mut pair| {
                                                            match selected_side.clone().unwrap_ji() {
                                                                SelectedSide::One(side) => {
                                                                    match side {
                                                                        Side::Left => {
                                                                            pair.0.audio = Some(audio);
                                                                        },
                                                                        Side::Right => {
                                                                            pair.1.audio = Some(audio);
                                                                        }
                                                                    }
                                                                }
                                                                SelectedSide::Both => {
                                                                    pair.0.audio = Some(audio.clone());
                                                                    pair.1.audio = Some(audio);
                                                                }
                                                            }

                                                            pair
                                                        });
                                                    })),
                                                    Some(clone!(state, selected_side => move || {
                                                        state.base.replace_pair(idx, |mut pair| {
                                                            match selected_side.clone().unwrap_ji() {
                                                                SelectedSide::One(side) => {
                                                                    match side {
                                                                        Side::Left => {
                                                                            pair.0.audio = None;
                                                                        },
                                                                        Side::Right => {
                                                                            pair.1.audio = None;
                                                                        }
                                                                    }
                                                                }
                                                                SelectedSide::Both => {
                                                                    pair.0.audio = None;
                                                                    pair.1.audio = None;
                                                                }
                                                            }

                                                            pair
                                                        });
                                                    })),
                                                );

                                                Some(AudioInput::render(AudioInput::new(opts, callbacks), None))
                                            } else {
                                                Some(render_empty_audio())
                                            }
                                            /* match audio {
                                                Some(audio) => Some(AudioInput::render(audio.clone(), None)),
                                                None => Some(render_empty_audio()),
                                            } */
                                            // None
                                        })))
                                    }))
                                },
                                _ => None,
                            }
                        })))
                    })
                )
            }))
        })))
    })
}

fn render_empty_audio() -> Dom {
    html!("sidebar-empty", {
        .property("label", STR_EMPTY_AUDIO_SELECTION)
    })
}

fn render_non_empty<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("sidebar-empty", {
        .property("label", STR_NONEMPTY_LIST_LABEL)
        .child(
            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .property("color", "blue")
                .text(strings::STR_CREATE_NEW_LIST)
                .event(clone!(state => move |_evt:events::Click| {
                    state.base.clear_all();
                }))
            })
        )
    })
}

fn render_tab<RawData: RawDataExt, E: ExtraExt>(
    state: Rc<Step1<RawData, E>>,
    tab_kind: MenuTabKind,
    idx: usize,
    enabled: bool,
) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            enabled,
            clone!(state => move || state.tab_index.signal_ref(move |current_tab_idx| {
                current_tab_idx.as_ref().map_or(false, |current_tab_idx| *current_tab_idx == idx)
            })),
            clone!(state => move || {
                state.tab_index.set_neq(Some(idx))
            }),
        ),
        Some("tabs"),
    )
}
