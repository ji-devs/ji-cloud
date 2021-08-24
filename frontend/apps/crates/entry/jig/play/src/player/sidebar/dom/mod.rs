use std::rc::Rc;

use components::module::_common::thumbnail::ModuleThumbnail;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::player::sidebar::actions::load_ages;

use super::state::State;

pub(super) mod info;
pub(super) mod report;
pub(super) mod share;

pub fn render(state: Rc<State>) -> Dom {
    load_ages(Rc::clone(&state));

    html!("jig-play-sidebar", {
        .property("slot", "sidebar")
        .property_signal("jigName", state.player_state.jig.signal_cloned().map(|jig| {
            match jig {
                None => String::new(),
                Some(jig) => jig.display_name,
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
        .child(html!("jig-play-sidebar-action", {
            .property("slot", "actions")
            .property("kind", "like")
        }))
        .child_signal(state.player_state.jig.signal_ref(clone!(state => move |jig| {
            // only show share options if jig is published
            match jig {
                Some(jig) if jig.publish_at.is_some() => {
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
                    Some(html!("empty-fragment", {
                        .property("slot", "modules")
                        .children(jig.modules.iter().enumerate().map(|(i, module)| {
                            html!("jig-sidebar-module", {
                                .property("module", module.kind.as_str())
                                .property("index", i as u32)
                                .property("selected", true)
                                .property_signal("selected", state.player_state.active_module.signal().map(move |active_module_index| {
                                    i == active_module_index
                                }))
                                .event(clone!(state => move |_: events::Click| {
                                    state.player_state.active_module.set(i);
                                }))
                                .child(ModuleThumbnail::render_live(
                                    Rc::new(ModuleThumbnail {
                                        jig_id: state.player_state.jig_id.clone(),
                                        module: module.clone(), 
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
