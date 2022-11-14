use super::{actions, sidebar};
use components::audio::mixer::audio_iframe_messenger;
use components::overlay::handle::OverlayHandle;
use components::share_asset::ShareAsset;
use dominator::{clone, html, with_node, Dom};
use dominator_helpers::{events::Message, signals::DefaultSignal};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use js_sys::Reflect;
use shared::domain::module::body::InstructionsType;
use shared::domain::{jig::JigResponse, module::ModuleKind};
use std::collections::HashMap;
use std::rc::Rc;
use utils::events;
use utils::iframe::{AssetPlayerToPlayerPopup, IframeMessageExt};
use utils::init::analytics;
use utils::js_wrappers::is_iframe;
use utils::prelude::is_in_iframe;
use utils::{
    iframe::{IframeAction, ModuleToJigPlayerMessage},
    prelude::SETTINGS,
    routes::{ModuleRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, HtmlIFrameElement};

use super::state::JigPlayer;

const DEFAULT_INSTRUCTIONS_TEXT: &str = "1, 2, 3 Go!";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShowInstructions {
    AudioOnly,
    All,
}

impl JigPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        actions::load_data(state.clone());

        let should_show_instructions = map_ref! {
            let instructions = state.instructions.signal_cloned(),
            let timer = state.timer.signal_cloned(),
            let started = state.started.signal_cloned().dedupe()
            => {
                if *started {
                    if let Some(instructions) = instructions {
                        if timer.is_some() || instructions.text.is_some() {
                            let is_instructions = instructions.instructions_type.is_instructions();
                            // if there is text or a timer, and
                            if is_instructions || instructions.text.is_some() {
                                // this is an instructions type or there is text
                                Some(ShowInstructions::All)
                            } else if instructions.audio.is_some() {
                                // otherwise play just audio
                                Some(ShowInstructions::AudioOnly)
                            } else {
                                None
                            }
                        } else if instructions.audio.is_some() {
                            // otherwise, if there is just audio, then play that
                            Some(ShowInstructions::AudioOnly)
                        } else {
                            None
                        }
                    } else {
                        // No instructions have been set for this activity.
                        None
                    }
                } else {
                    None
                }
            }
        };

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
            .future(state.active_module.signal_cloned().for_each(clone!(state => move |_active_module| {
                state.started.set_neq(false);
                state.instructions.set(None);
                state.instructions_visible.set_neq(false);
                async {}
            })))
            .future(should_show_instructions.for_each(clone!(state => move |should_show| {
                match should_show {
                    Some(ShowInstructions::AudioOnly) => {
                        actions::play_instructions_audio(state.clone());
                    }
                    Some(ShowInstructions::All) => {
                        // Only show, never hide from here. Otherwise we can cause a race condition between Play and Pause.
                        actions::show_instructions(state.clone(), true);
                    }
                    _ => {

                    }
                }
                async {}
            })))
            .prop_signal("rtl", state.jig.signal_cloned().map(|jig| {
                jig.map(|jig| jig.jig_data.default_player_settings.direction.is_rtl())
            }))
            .prop_signal("paused", state.paused.signal())
            .prop_signal("isLegacy", state.jig.signal_ref(|jig| {
                if let Some(jig) = jig {
                    if let Some(first_module) = jig.jig_data.modules.get(0) {
                        if first_module.kind == ModuleKind::Legacy {
                            return true;
                        }
                    }
                };
                false
            }))
            .prop("inIframe", is_in_iframe())
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
                        .prop("slot", "indicators")
                        .prop_signal("value", state.points.signal())
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
                            .prop("slot", "background")
                            .prop_signal("playing", state.bg_audio_playing.signal())
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
                        .prop("slot", "message")
                    }))
                } else {
                    None
                }
            }))
            .child_signal(active_module_valid_signal(Rc::clone(&state)).map(clone!(state => move |valid| {
                if valid {
                    Some(html!("iframe" => HtmlIFrameElement, {
                        .prop("allow", "autoplay; fullscreen")
                        .prop("slot", "iframe")
                        .prop_signal("src", jig_and_active_module_signal(Rc::clone(&state)).map(clone!(state => move|(jig, active_module_index)| {
                            match (jig, active_module_index) {
                                (Some(jig), Some(active_module_index)) => {
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
                                (Some(_jig), None) => "".to_owned(),
                                _ => {
                                    crate::debug::settings().empty_module_url.to_string()
                                },
                            }
                        })))
                        .after_inserted(clone!(state => move|element| {
                            *state.iframe.borrow_mut() = Some(element);
                        }))
                        .apply(audio_iframe_messenger)
                    }))
                } else {
                    None
                }
            })))
            .child_signal(state.instructions.signal_cloned().map(clone!(state => move |instructions| {
                instructions.map(clone!(state => move |instructions| {
                    html!("empty-fragment", {
                        .prop("slot", "instructions")
                        .child(html!("button-icon", {
                            .style("width", "40px")
                            .style("height", "40px")
                            .prop("iconPath", "jig/play/icn-instructions.svg")
                            .prop("iconHoverPath", "jig/play/icn-instructions-hover.svg")
                            .event(clone!(state => move |_evt: events::Click| {
                                let instructions = state.instructions.get_cloned();
                                let timer = state.timer.get_cloned();
                                if let Some(instructions) = instructions {
                                    if timer.is_some() || instructions.text.is_some() {
                                        actions::show_instructions(state.clone(), true);
                                    } else if instructions.audio.is_some() {
                                        actions::play_instructions_audio(state.clone());
                                    }
                                }
                            }))
                        }))
                        .child_signal(state.instructions_visible.signal_ref(clone!(state, instructions => move |visible| {
                            if *visible {
                                Some(html!("empty-fragment" => HtmlElement, {
                                    .with_node!(elem => {
                                        .apply(OverlayHandle::lifecycle(
                                            clone!(state, instructions => move || {
                                                html!("overlay-tooltip-info", {
                                                    .prop("centeredContent", true)
                                                    .prop("marginX", -16)
                                                    .prop("target", &elem)
                                                    .attr("targetAnchor", "br")
                                                    .attr("contentAnchor", "oppositeV")
                                                    .prop("size", "large")
                                                    .prop("color", "dark-blue")
                                                    .apply(clone!(instructions => move |dom| {
                                                        match instructions.instructions_type {
                                                            InstructionsType::Instructions => {
                                                                dom.prop("body", &instructions.text.unwrap_or(DEFAULT_INSTRUCTIONS_TEXT.to_owned()))
                                                            }
                                                            InstructionsType::Feedback => {
                                                                if let Some(text) = &instructions.text {
                                                                    dom.prop("body", text)
                                                                } else {
                                                                    dom
                                                                }
                                                            }
                                                        }
                                                    }))
                                                    .prop("closeable", true)
                                                    .prop("strategy", "track")
                                                    .event(clone!(state => move |_evt: events::Close| {
                                                        actions::show_instructions(state.clone(), false);
                                                    }))
                                                    .apply_if(instructions.audio.is_some(), clone!(state => move |dom| {
                                                        let hover = Mutable::new(false);
                                                        dom.child(html!("button-rect", {
                                                            .prop("slot", "actions")
                                                            .prop("kind", "text")
                                                            .prop("color", "lightBlue")
                                                            .prop("size", "small")
                                                            .event(clone!(hover => move |_evt: events::PointerEnter| hover.set_neq(true)))
                                                            .event(clone!(hover => move |_evt: events::PointerLeave| hover.set_neq(false)))
                                                            .child(html!("img-ui", {
                                                                .style("height", "24px")
                                                                .prop_signal("path", hover.signal_ref(|hover| {
                                                                    if *hover {
                                                                        "jig/play/icon-repeat-hover.svg"
                                                                    } else {
                                                                        "jig/play/icon-repeat.svg"
                                                                    }
                                                                }))
                                                            }))
                                                            .text("Repeat")
                                                            .event(clone!(state => move |_evt: events::Click| {
                                                                actions::play_instructions_audio(state.clone());
                                                            }))
                                                        }))
                                                    }))
                                                    .apply(clone!(state => move |dom| {
                                                        let hover = Mutable::new(false);
                                                        dom.child(html!("button-rect", {
                                                            .prop("slot", "actions")
                                                            .prop("kind", "filled")
                                                            .prop("color", "blue")
                                                            .prop("size", "small")
                                                            .style("margin-left", "auto")
                                                            .event(clone!(hover => move |_evt: events::PointerEnter| hover.set_neq(true)))
                                                            .event(clone!(hover => move |_evt: events::PointerLeave| hover.set_neq(false)))
                                                            .child(html!("img-ui", {
                                                                .style("height", "24px")
                                                                .prop_signal("path", hover.signal_ref(|hover| {
                                                                    if *hover {
                                                                        "jig/play/icon-thumbsup-hover.svg"
                                                                    } else {
                                                                        "jig/play/icon-thumbsup.svg"
                                                                    }
                                                                }))
                                                            }))
                                                            .text("OK")
                                                            .event(clone!(state => move |_evt: events::Click| {
                                                                actions::show_instructions(state.clone(), false);
                                                            }))
                                                        }))
                                                    }))
                                                })
                                            })
                                        ))
                                    })
                                }))
                            } else {
                                None
                            }
                        })))
                    })
                }))
            })))
            .children(&mut [
                html!("jig-play-play-button", {
                    .prop("slot", "play-button")
                }),
                html!("jig-play-play-pause", {
                    .prop("slot", "play-pause-button")
                    .prop_signal("mode", state.paused.signal().map(|paused| {
                        match paused {
                            true =>  "play",
                            false =>  "pause",
                        }
                    }))
                    .event(clone!(state => move |_:events::Click| {
                        actions::toggle_paused(&state);
                    }))
                }),
                html!("jig-play-move-button", {
                    .prop("slot", "back")
                    .prop("kind", "back")
                    .visible_signal(jig_and_active_module_signal(Rc::clone(&state)).map(|(jig, active_module)| {
                        // if module already loaded and not first module
                        match (jig, active_module) {
                            (Some(_jig), Some(active_module)) => {
                                active_module != 0
                            },
                            _ => false,
                        }
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        actions::navigate_back(Rc::clone(&state));
                    }))
                    .global_event(clone!(state => move |e: events::KeyUp| {
                        if &e.key() == "ArrowLeft" {
                            actions::navigate_back(Rc::clone(&state));
                        }
                    }))
                }),
                html!("jig-play-progress-bar", {
                    .prop("slot", "progress")
                    .prop_signal("percent", progress_signal(state.clone()))
                }),
                html!("jig-play-move-button", {
                    .prop("slot", "forward")
                    .prop("kind", "forward")
                    .event(clone!(state => move |_: events::Click| {
                        actions::navigate_forward(Rc::clone(&state));
                    }))
                    .global_event(clone!(state => move |e: events::KeyUp| {
                        if &e.key() == "ArrowRight" {
                            actions::navigate_forward(Rc::clone(&state));
                        }
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
        match (jig, active_module_index) {
            (Some(jig), Some(active_module_index)) => {
                match &jig.jig_data.modules.get(active_module_index) {
                    Some(module) => module.is_complete || matches!(module.kind, ModuleKind::Legacy),
                    None => true, // Active module isn't in the list
                }
            }
            _ => true, // Jig isn't set
        }
    })
}

fn jig_and_active_module_signal(
    state: Rc<JigPlayer>,
) -> impl Signal<Item = (Option<JigResponse>, Option<usize>)> {
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
                if let Some(active_module_index) = active_module_index {
                    let len = jig.jig_data.modules.len();
                    let step_percent = 100f32 / len as f32;
                    let current_progress = active_module_index as f32 * step_percent;
                    // TODO: ask corrine if this should be here
                    let current_progress = current_progress + step_percent;
                    log::info!("{}", current_progress);
                    current_progress.round() as u32
                } else {
                    0
                }
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
                    .prop("slot", "dialog")
                    .prop("open", true)
                    .prop("autoClose", false)
                    .child(html!("jig-play-done-popup", {
                        .apply(|mut dom| {
                            if state.player_options.display_score {
                                dom = dom.prop_signal("score", state.points.signal());
                            };
                            if !state.player_options.track_assessments {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .prop("slot", "actions")
                                        .prop("kind", "replay")
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
                                dom = dom.child_signal(state.jig.signal_cloned().map(|jig| {
                                    jig.map(|jig| {
                                        ShareAsset::new(jig.into()).render(
                                            html!("jig-play-done-action", {
                                                .text("share")
                                                .prop("kind", "share")
                                            }),
                                            Some("actions")
                                        )
                                    })
                                }));
                            }
                            if is_iframe() {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .prop("slot", "actions")
                                        .prop("kind", "exit")
                                        .text("exit")
                                        .event(|_: events::Click| {
                                            let _ = IframeAction::new(AssetPlayerToPlayerPopup::Close).try_post_message_to_parent();
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
                    .prop("slot", "dialog")
                    .prop("open", true)
                    .prop("autoClose", false)
                    .child(html!("jig-play-time-up-popup", {
                        .apply(|mut dom| {
                            if !state.player_options.track_assessments {
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .prop("slot", "actions")
                                        .prop("kind", "replay")
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
                    .prop("slot", "indicators")
                    .prop_signal("value", timer.time.signal().map(|time| {
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
