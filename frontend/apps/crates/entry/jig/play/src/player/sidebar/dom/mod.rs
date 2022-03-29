use std::rc::Rc;

use components::module::_common::thumbnail::ModuleThumbnail;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::player::sidebar::actions::load_ages;

use super::super::state::can_load_liked_status;
use super::state::State;

pub(super) mod info;
pub(super) mod like;
pub(super) mod report;
pub(super) mod share;

pub fn render(state: Rc<State>) -> Dom {
    load_ages(Rc::clone(&state));

    html!("jig-play-sidebar", {
        .property("slot", "sidebar")
        .property_signal("jigName", state.player_state.jig.signal_cloned().map(|jig| {
            match jig {
                None => String::new(),
                Some(jig) => jig.jig_data.display_name,
            }
        }))
        .property_signal("open", state.sidebar_open.signal())
        .child(html!("button-empty", {
            .property("slot", "close")
            .text("<")
            .event(clone!(state => move |_: events::Click| {
                state.sidebar_open.set(false);
            }))
        }))
        .child(html!("button", {
            .property("slot", "opener")
            .event(clone!(state => move |_: events::Click| {
                state.sidebar_open.set(true);
            }))
        }))
        .child(html!("button", {
            .property("slot", "closer")
            .text("<")
            .event(clone!(state => move |_: events::Click| {
                state.sidebar_open.set(false);
            }))
        }))
        .child_signal(state.player_state.jig.signal_ref(clone!(state => move |jig| {
            match jig {
                Some(jig) => {
                    if can_load_liked_status(jig) {
                        Some(like::render(Rc::clone(&state), jig))
                    } else {
                        None
                    }
                },
                _ => None,
            }
        })))
        .child_signal(state.player_state.jig.signal_ref(clone!(state => move |jig| {
            // only show share options if jig is published
            match jig {
                Some(jig) if jig.jig_data.draft_or_live.is_live() => {
                    Some(share::render(Rc::clone(&state)))
                },
                _ => None,
            }
        })))
        .child(info::render(Rc::clone(&state)))
        .child_signal(state.player_state.jig.signal_cloned().map(clone!(state => move|jig| {
            match jig {
                None => None,
                Some(jig) => {
                    let module_count = jig.jig_data.modules.len();

                    Some(html!("empty-fragment", {
                        .property("slot", "modules")
                        .children(jig.jig_data.modules.iter().enumerate().map(|(i, module)| {
                            html!("jig-sidebar-module", {
                                .property("module", module.kind.as_str())
                                .property("index", i as u32)
                                .property("isLastModule", i == module_count - 1)
                                .property("selected", true)
                                .property_signal("selected", state.player_state.active_module.signal().map(move |active_module_index| {
                                    i == active_module_index
                                }))
                                .event(clone!(state => move |_: events::Click| {
                                    state.player_state.active_module.set(i);
                                }))
                                .child(ModuleThumbnail::render_live(
                                    Rc::new(ModuleThumbnail {
                                        jig_id: state.player_state.jig_id,
                                        module: Some(module.clone()),
                                        is_jig_fallback: false,
                                    }),
                                    Some("window")
                                ))
                            })
                        }).collect::<Vec<Dom>>())
                    }))
                },
            }
        })))
    })
}
