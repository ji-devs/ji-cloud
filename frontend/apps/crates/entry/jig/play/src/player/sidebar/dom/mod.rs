use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::player::sidebar::actions::load_ages;

use super::state::State;

pub(super) mod share;
pub(super) mod info;
pub(super) mod report;


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
        .children(&mut [
            html!("jig-play-sidebar-action", {
                .property("slot", "actions")
                .property("kind", "like")
            }),
            share::render(Rc::clone(&state)),
            info::render(Rc::clone(&state)),
        ])
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
                                .children(&mut [
                                    html!("img-ji", {
                                        .property("slot", "window")
                                        .property("size", "original")
                                        .property("lib", "mock")
                                        .property("id", "something.jpg")
                                    }),
                                ])
                            })
                        }).collect::<Vec<Dom>>())
                    }))
                },
            }
        })))
    })
}
