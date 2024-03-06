use std::collections::HashMap;
use std::rc::Rc;

use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;
use utils::init::analytics;

use super::super::state::can_load_liked_status;
use super::state::Sidebar;

pub(super) mod info;
pub(super) mod like;
pub(super) mod report;
pub(super) mod share;

impl Sidebar {
    pub fn render(self: &Rc<Sidebar>) -> Dom {
        let state = self;
        state.load_ages();

        html!("jig-play-sidebar", {
            .prop("slot", "sidebar")
            .prop_signal("jigName", state.player_state.jig.signal_cloned().map(|jig| {
                match jig {
                    None => String::new(),
                    Some(jig) => jig.jig_data.display_name,
                }
            }))
            .prop_signal("open", state.sidebar_open.signal())
            .child(html!("button-empty", {
                .prop("slot", "close")
                .text("<")
                .event(clone!(state => move |_: events::Click| {
                    state.sidebar_open.set(false);
                }))
            }))
            .child(html!("button", {
                .prop("slot", "opener")
                .event(clone!(state => move |_: events::Click| {
                    state.sidebar_open.set(true);
                    analytics::event("Jig Play Sidebar Jiggling", None);
                }))
            }))
            .child(html!("button", {
                .prop("slot", "closer")
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
                        Some(state.render_share(jig.clone()))
                    },
                    _ => None,
                }
            })))
            .child(state.render_info())
            .child_signal(state.player_state.jig.signal_cloned().map(clone!(state => move|jig| {
                match jig {
                    None => None,
                    Some(jig) => {
                        let module_count = jig.jig_data.modules.len();

                        Some(html!("empty-fragment", {
                            .prop("slot", "modules")
                            .children(jig.jig_data.modules.iter().enumerate().map(|(i, module)| {
                                html!("jig-sidebar-module", {
                                    .prop("module", module.kind.as_str())
                                    .prop("index", i as u32)
                                    .prop("isLastModule", i == module_count - 1)
                                    .prop("selected", true)
                                    .prop_signal("selected", state.player_state.active_module.signal().map(move |active_module_index| {
                                        Some(i) == active_module_index
                                    }))
                                    .event(clone!(state => move |_: events::Click| {
                                        state.player_state.active_module.set(Some(i));
                                    }))
                                    .child(
                                        ModuleThumbnail::new(
                                            state.player_state.jig_id.into(),
                                            Some(module.clone()),
                                            ThumbnailFallback::Module,
                                            state.player_state.draft_or_live,
                                        ).render_live(Some("window"))
                                    )
                                })
                            }).collect::<Vec<Dom>>())
                        }))
                    },
                }
            })))
        })
    }

    fn track_action(self: &Rc<Self>, action: &str) {
        // Don't unwrap the jig field because we don't want analytics logic to break the app.
        if let Some(jig) = self.player_state.jig.get_cloned() {
            let mut properties = HashMap::new();
            properties.insert("Jig ID", jig.id.0.to_string());
            properties.insert("Jig Name", jig.jig_data.display_name);
            properties.insert("Action", action.to_owned());

            analytics::event("Jig Play Sidebar Action", Some(properties));
        }
    }
}
