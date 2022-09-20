use super::{actions, sidebar};
use components::share_asset::ShareAsset;
use dominator::{clone, events, html, with_node, Dom};
use dominator_helpers::{events::Message, signals::DefaultSignal};
use futures_signals::map_ref;
use futures_signals::signal::{Signal, SignalExt};
use js_sys::Reflect;
use shared::domain::{jig::JigResponse, module::ModuleKind};
use std::collections::HashMap;
use std::rc::Rc;
use utils::iframe::{AssetPlayerToPlayerPopup, IframeMessageExt};
use utils::init::analytics;
use utils::js_wrappers::is_iframe;
use utils::{
    iframe::{IframeAction, ModuleToJigPlayerMessage},
    prelude::SETTINGS,
    routes::{ModuleRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, HtmlIFrameElement};

use super::state::JigPlayer;

impl JigPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        actions::load_data(state.clone());

        html!("jig-play-landing", {
            .future(state.jig.signal_cloned().for_each(clone!(state => move |jig| {
                // Don't unwrap the jig field because we don't want analytics logic to break the app.
                if let Some(jig) = jig {
                    let mut properties = HashMap::new();

                    if state.player_options.is_student {
                        properties.insert("IsStudent", "Yes".to_owned());
                    } else {
                        properties.insert("IsTeacher", "Yes".to_owned());
                    };

                    properties.insert("Jig ID", jig.id.0.to_string());
                    properties.insert("Jig Name", jig.jig_data.display_name);

                    analytics::event("Jig Play", Some(properties));
                }

                async {}
            })))
            .property_signal("rtl", state.jig.signal_cloned().map(|jig| {
                jig.map(|jig| jig.jig_data.default_player_settings.direction.is_rtl())
            }))
            .property_signal("paused", state.paused.signal())
            .property_signal("isLegacy", state.jig.signal_ref(|jig| {
                if let Some(jig) = jig {
                    if let Some(first_module) = jig.jig_data.modules.get(0) {
                        if first_module.kind == ModuleKind::Legacy {
                            return true;
                        }
                    }
                };
                false
            }))
            .global_event(clone!(state => move |evt:Message| {
                match evt.try_serde_data::<IframeAction<ModuleToJigPlayerMessage>>() {
                    Err(_) => {},
                    Ok(m) => {
                        actions::on_iframe_message(Rc::clone(&state), m.data)
                    },
                };
            }))
            .apply(|dom| {
                if state.player_options.is_student {
                    dom
                } else {
                    let sidebar_state = Rc::new(sidebar::state::State::new(state.clone()));
                    dom.child(sidebar::dom::render(sidebar_state))
                }
            })
            .apply(clone!(state => move|dom| {
                if state.player_options.display_score {
                    dom.child(html!("jig-play-points-indicator", {
                        .visible(state.player_options.display_score)
                        .property("slot", "indicators")
                        .property_signal("value", state.points.signal())
                    }))
                } else {
                    dom
                }
            }))
            .child_signal(state.jig.signal_ref(clone!(state => move |jig| {
                match &jig {
                    // Only render the background music element on a jig if the jig has background
                    // music configured.
                    Some(jig) if jig.jig_data.audio_background.is_some() => {
                        Some(html!("jig-play-background-music", {
                            .property("slot", "background")
                            .property_signal("playing", state.bg_audio_playing.signal())
                            .event(clone!(state => move|_: events::Click| {
                                actions::toggle_background_audio(Rc::clone(&state));
                            }))
                        }))
                    },
                    _ => None
                }
            })))
            .child_signal(active_module_valid_signal(Rc::clone(&state)).map(|valid| {
                if !valid {
                    Some(html!("main-empty", {
                        .property("slot", "message")
                    }))
                } else {
                    None
                }
            }))
            .child_signal(active_module_valid_signal(Rc::clone(&state)).map(clone!(state => move |valid| {
                if valid {
                    Some(html!("iframe" => HtmlIFrameElement, {
                        .property("allow", "autoplay; fullscreen")
                        .property("slot", "iframe")
                        .property_signal("src", jig_and_active_module_signal(Rc::clone(&state)).map(clone!(state => move|(jig, active_module_index)| {
                            match jig {
                                None => {
                                    crate::debug::settings().empty_module_url.to_string()
                                },
                                Some(jig) => {
                                    let active_module = &jig.jig_data.modules.get(active_module_index);

                                    match active_module {
                                        // module doesn't exist, can happen with jigs that don't have a cover
                                        None => String::new(),
                                        Some(active_module) => {
                                            let mut route: String = Route::Module(ModuleRoute::Play(
                                                active_module.kind,
                                                state.jig_id.into(),
                                                active_module.id
                                            )).into();

                                            if state.player_options.draft_or_live.is_draft() {
                                                route = format!("{}?draft_or_live=draft", route);
                                            }

                                            let url = unsafe {
                                                SETTINGS.get_unchecked()
                                                    .remote_target
                                                    .spa_iframe(&route)
                                            };
                                            url
                                        },
                                    }
                                },
                            }
                        })))
                        .after_inserted(clone!(state => move|element| {
                            *state.iframe.borrow_mut() = Some(element);
                        }))
                    }))
                } else {
                    None
                }
            })))
            .children(&mut [
                html!("jig-play-play-button", {
                    .property("slot", "play-button")
                }),
                html!("jig-play-play-pause", {
                    .property("slot", "play-pause-button")
                    .property_signal("mode", state.paused.signal().map(|paused| {
                        match paused {
                            true =>  "play",
                            false =>  "pause",
                        }
                    }))
                    .event(clone!(state => move |_:events::Click| {
                        actions::toggle_paused(Rc::clone(&state));
                    }))
                }),
                html!("jig-play-move-button", {
                    .property("slot", "back")
                    .property("kind", "back")
                    .visible_signal(jig_and_active_module_signal(Rc::clone(&state)).map(|(jig, active_module)| {
                        // if module already loaded and not first module
                        match jig {
                            None => false,
                            Some(_jig) => {
                                active_module != 0
                            },
                        }
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        actions::navigate_back(Rc::clone(&state));
                    }))
                }),
                html!("jig-play-progress-bar", {
                    .property("slot", "progress")
                    .property_signal("percent", progress_signal(state.clone()))
                }),
                html!("jig-play-move-button", {
                    .property("slot", "forward")
                    .property("kind", "forward")
                    .event(clone!(state => move |_: events::Click| {
                        actions::navigate_forward(Rc::clone(&state));
                    }))
                }),
            ])
            .child_signal(render_time_indicator(Rc::clone(&state)))
            .child_signal(render_done_popup(Rc::clone(&state)))
            .child_signal(render_time_up_popup(Rc::clone(&state)))
        })
    }
}

/// Emits `true` if the module doesn't exist in the list of modules or if the jig is `None`.
/// Otherwise emits the value of the module's `is_complete` field.
fn active_module_valid_signal(state: Rc<JigPlayer>) -> impl Signal<Item = bool> {
    jig_and_active_module_signal(state).map(|(jig, active_module_index)| {
        match jig {
            Some(jig) => match &jig.jig_data.modules.get(active_module_index) {
                Some(module) => module.is_complete || matches!(module.kind, ModuleKind::Legacy),
                None => true, // Active module isn't in the list
            },
            None => true, // Jig isn't set
        }
    })
}

fn jig_and_active_module_signal(
    state: Rc<JigPlayer>,
) -> impl Signal<Item = (Option<JigResponse>, usize)> {
    map_ref! {
        let jig = state.jig.signal_cloned(),
        let active_module = state.active_module.signal_cloned() => (
            jig.clone(), *active_module
        )
    }
}

fn ten_sec_signal(state: Rc<JigPlayer>) -> impl Signal<Item = bool> {
    state
        .timer
        .signal_cloned()
        .map(|timer| {
            DefaultSignal::new(
                false,
                timer.map(|timer| timer.time.signal().map(|time| time == 10)),
            )
        })
        .flatten()
}

fn progress_signal(state: Rc<JigPlayer>) -> impl Signal<Item = u32> {
    (map_ref! {
        let active_module = state.active_module.signal(),
        let jig = state.jig.signal_cloned() =>
            (*active_module, jig.clone())
    })
    .map(move |(active_module_index, jig)| {
        match jig {
            None => 0,
            Some(jig) => {
                let len = jig.jig_data.modules.len();
                let step_percent = 100f32 / len as f32;
                let current_progress = active_module_index as f32 * step_percent;
                // TODO: ask corrine if this should be here
                let current_progress = current_progress + step_percent;
                log::info!("{}", current_progress);
                current_progress.round() as u32
            }
        }
    })
}

fn render_done_popup(state: Rc<JigPlayer>) -> impl Signal<Item = Option<Dom>> {
    state.done.signal().map(clone!(state => move |done| {
        match done {
            false => None,
            true => {
                Some(html!("dialog-overlay", {
                    .property("slot", "dialog")
                    .property("open", true)
                    .property("autoClose", false)
                    .child(html!("jig-play-done-popup", {
                        .apply(|mut dom| {
                            if state.player_options.display_score {
                                dom = dom.property_signal("score", state.points.signal());
                            };
                            if !state.player_options.track_assessments {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .property("slot", "actions")
                                        .property("kind", "replay")
                                        .event(clone!(state => move |_: events::Click| {
                                            actions::navigate_to_index(
                                                Rc::clone(&state),
                                                0
                                            );
                                        }))
                                    })
                                );
                            }
                            if !state.player_options.is_student {
                                dom = dom.child(ShareAsset::new(state.jig_id.into()).render(
                                    html!("jig-play-done-action", {
                                        .text("share")
                                        .property("kind", "share")
                                    }),
                                    Some("actions")
                                ));
                            }
                            if is_iframe() {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .property("slot", "actions")
                                        .property("kind", "exit")
                                        .text("exit")
                                        .event(|_: events::Click| {
                                            let e = IframeAction::new(AssetPlayerToPlayerPopup::Close).try_post_message_to_parent();
                                            log::info!("{:?}", e);
                                        })
                                    })
                                );
                            }
                            dom
                        })
                    }))
                }))
            },
        }
    }))
}

fn time_up_signal(state: Rc<JigPlayer>) -> impl Signal<Item = bool> {
    state
        .timer
        .signal_cloned()
        .map(|timer| {
            DefaultSignal::new(
                false,
                timer.map(|timer| timer.time.signal().map(|time| time == 0)),
            )
        })
        .flatten()
}

fn render_time_up_popup(state: Rc<JigPlayer>) -> impl Signal<Item = Option<Dom>> {
    time_up_signal(Rc::clone(&state)).map(clone!(state => move |time_up| {
        match time_up {
            false => None,
            true => {
                Some(html!("dialog-overlay", {
                    .property("slot", "dialog")
                    .property("open", true)
                    .property("autoClose", false)
                    .child(html!("jig-play-time-up-popup", {
                        .apply(|mut dom| {
                            if !state.player_options.track_assessments {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .property("slot", "actions")
                                        .property("kind", "replay")
                                        .event(clone!(state => move |_: events::Click| {
                                            actions::reload_iframe(Rc::clone(&state));
                                        }))
                                    })
                                );
                            }
                            dom
                        })
                    }))
                }))
            }
        }
    }))
}

fn render_time_indicator(state: Rc<JigPlayer>) -> impl Signal<Item = Option<Dom>> {
    state.timer.signal_cloned().map(clone!(state => move |timer| {
        match timer {
            None => None,
            Some(timer) => {
                Some(html!("jig-play-timer-indicator" => HtmlElement, {
                    .property("slot", "indicators")
                    .property_signal("value", timer.time.signal().map(|time| {
                        let minutes = (time as f32 / 60.0).floor();
                        let seconds = time % 60;
                        format!("{}:{:0>2}", minutes, seconds)
                    }))
                    .with_node!(elem => {
                        .future(ten_sec_signal(Rc::clone(&state)).for_each(move |less_than_10_sec| {
                            if less_than_10_sec {
                                let buzz_method = Reflect::get(
                                    &elem,
                                    &JsValue::from_str("buzz")
                                )
                                    .unwrap_ji();
                                log::info!("{:?}", buzz_method);
                                let buzz_method = buzz_method.dyn_ref::<js_sys::Function>().unwrap_ji();
                                let _ = buzz_method.call0(&elem);
                            }
                            async {}
                        }))
                    })
                }))
            }
        }
    }))
}
