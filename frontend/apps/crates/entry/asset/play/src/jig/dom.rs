use super::sidebar::Sidebar;
use components::audio::mixer::audio_iframe_messenger;
use components::overlay::handle::OverlayHandle;
use components::share_asset::ShareAsset;
use dominator::{clone, html, with_node, Dom};
use dominator_helpers::{events::Message, signals::DefaultSignal};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use gloo::utils::{body, document};
use js_sys::Reflect;
use shared::domain::module::body::ModuleAssistType;
use shared::domain::{jig::JigResponse, module::ModuleKind};
use std::collections::HashMap;
use std::rc::Rc;
use utils::init::analytics;
use utils::js_wrappers::is_iframe;
use utils::keyboard::KeyEvent;
use utils::prelude::is_in_iframe;
use utils::routes::HomeRoute;
use utils::{dialog, events};
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
const STR_EMPTY: &str = "Oops! Looks like you have some empty content in your deck!";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShowAssist {
    AudioOnly,
    All,
}

impl JigPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        // Timer is not Copy and cannot be deduped, so any change to timer, even if it is the same value
        // will trigger an updated which will cause the instructions popup to show again when it is not
        // supposed to.
        let has_timer = state
            .timer
            .signal_cloned()
            .map(|timer| timer.is_some())
            .dedupe();

        let should_show_assist = map_ref! {
            let module_assist = state.module_assist.signal_cloned(),
            let has_timer = has_timer,
            let started = state.started.signal_cloned().dedupe()
            => {
                if *started {
                    if let Some(module_assist) = module_assist {
                        if *has_timer || module_assist.always_show || module_assist.text.is_some() {
                        // if module_assist.module_assist_type.is_instructions() || module_assist.text.is_some() {
                            let is_instructions = module_assist.module_assist_type.is_instructions();
                            // if there is text or a timer, and
                            if is_instructions || module_assist.text.is_some() {
                                // this is an instructions type or there is text
                                Some(ShowAssist::All)
                            } else if module_assist.audio.is_some() {
                                // otherwise play just audio
                                Some(ShowAssist::AudioOnly)
                            } else {
                                None
                            }
                        } else if module_assist.audio.is_some() {
                            // otherwise, if there is just audio, then play that
                            Some(ShowAssist::AudioOnly)
                        } else {
                            None
                        }
                    } else {
                        // No module assist has been set for this activity.
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

                    if state.is_student {
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
                state.module_assist.set(None);
                state.module_assist_visible.set_neq(false);
                async {}
            })))
            .future(should_show_assist.for_each(clone!(state => move |should_show| {
                match should_show {
                    Some(ShowAssist::AudioOnly) => {
                        // state.play_assist_audio();
                        // Workaround - play assist audio before other audio is played
                        state.show_assist(true);
                    }
                    Some(ShowAssist::All) => {
                        // Only show, never hide from here. Otherwise we can cause a race condition between Play and Pause.
                        state.show_assist(true);
                    }
                    _ => {
                        // Always drop the audio_handle whenever this signal fires and nothing should be shown/played
                        *state.module_assist_audio_handle.borrow_mut() = None;
                    }
                }
                async {}
            })))
            // Use state.player_options, not jig.jig_data.default_player_settings
            .prop_signal("rtl", state.direction.signal().map(|d| d.is_rtl()))
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
                        state.on_iframe_message(m.data)
                    },
                };
            }))
            .apply(|dom| {
                if state.is_student {
                    dom
                } else {
                    dom.child(Sidebar::new(&state).render())
                }
            })
            .child_signal(state.scoring.signal().map(clone!(state => move |scoring| {
                scoring.then(|| {
                    html!("div", {
                        .prop("slot", "indicators")
                        .style("display", "contents")
                        .child_signal(state.active_module_has_scoring().map(clone!(state => move |has_scoring| {
                            Some(html!("jig-play-points-indicator", {
                                .prop("hidden", !has_scoring)
                                .prop_signal("value", state.points.signal().map(|p| p * 100))
                            }))
                        })))
                    })
                })
            })))
            .apply_if(document().fullscreen_enabled(), clone!(state => move|dom| {
                dom.child(html!("jig-play-full-screen", {
                    .prop("slot", "full-screen")
                    .prop_signal("isFullScreen", state.is_full_screen.signal())
                    .event(clone!(state => move|_: events::Click| {
                        match state.is_full_screen.get() {
                            true => {
                                let _ = document().exit_fullscreen();
                            },
                            false => {
                                let _ = body().request_fullscreen();
                            },
                        };
                    }))
                    .global_event(clone!(state => move|_: events::FullScreenChange| {
                        let is_full_screen = document().fullscreen_element().is_some();
                        state.is_full_screen.set(is_full_screen);
                    }))
                }))
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
                                state.toggle_background_audio();
                            }))
                        }))
                    },
                    _ => None
                }
            })))
            .child_signal(state.active_module_valid_signal().map(|valid| {
                if !valid {
                    Some(html!("main-empty", {
                        .prop("slot", "message")
                        .prop("message", STR_EMPTY)
                    }))
                } else {
                    None
                }
            }))
            .child_signal(state.active_module_valid_signal().map(clone!(state => move |valid| {
                if valid {
                    Some(html!("iframe" => HtmlIFrameElement, {
                        .prop("allow", "autoplay; fullscreen")
                        .prop("slot", "iframe")
                        .prop_signal("src", state.jig_and_active_module_signal().map(clone!(state => move|(jig, active_module_index)| {
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

                                            if state.draft_or_live.is_draft() {
                                                route = format!("{}?draft_or_live=draft", route);
                                            }

                                            let url = SETTINGS.get()
                                                .unwrap_ji()
                                                .remote_target
                                                .spa_iframe(&route);
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
            .child_signal(state.module_assist.signal_cloned().map(clone!(state => move |module_assist| {
                module_assist.map(clone!(state => move |module_assist| {
                    html!("empty-fragment", {
                        .prop("slot", "module-assist")
                        .child(html!("button-icon", {
                            .style("width", "30px")
                            .style("height", "30px")
                            .prop("iconPath", "jig/play/icn-instructions.svg")
                            .prop("iconHoverPath", "jig/play/icn-instructions-hover.svg")
                            .event(clone!(state => move |_evt: events::Click| {
                                let module_assist = state.module_assist.get_cloned();
                                let timer = state.timer.get_cloned();
                                if let Some(module_assist) = module_assist {
                                    if ((timer.is_some() || module_assist.always_show) && module_assist.module_assist_type.is_instructions()) || module_assist.text.is_some() {
                                        // If there is a timer or assist is set to always_show, and the type is `Instructions`, or if there is text, show the popup
                                        state.show_assist(true);
                                    } else if module_assist.audio.is_some() {
                                        // state.play_assist_audio();
                                        // Workaround - play assist audio before other audio is played
                                        state.show_assist(true);
                                    }
                                }
                            }))
                        }))
                        .child_signal(state.module_assist_visible.signal_ref(clone!(state, module_assist => move |visible| {
                            if *visible {
                                Some(html!("empty-fragment" => HtmlElement, {
                                    .with_node!(elem => {
                                        .apply(OverlayHandle::lifecycle(
                                            clone!(state, module_assist => move || {
                                                let tooltip = match module_assist.is_audio_only() {
                                                    true => "overlay-tooltip-noop",
                                                    false => "overlay-tooltip-info",
                                                };
                                                log::info!("{}", tooltip);
                                                html!(tooltip, {
                                                    .prop("centeredContent", true)
                                                    .prop("marginX", -16)
                                                    .prop("target", &elem)
                                                    .attr("targetAnchor", "br")
                                                    .attr("contentAnchor", "oppositeV")
                                                    .prop("size", "large")
                                                    .prop("color", "dark-blue")
                                                    .prop("closeable", true)
                                                    .prop("strategy", "track")
                                                    .apply(clone!(module_assist => move |dom| {
                                                        match module_assist.module_assist_type {
                                                            ModuleAssistType::Instructions => {
                                                                dom.prop("body", &module_assist.text.unwrap_or(DEFAULT_INSTRUCTIONS_TEXT.to_owned()))
                                                            }
                                                            ModuleAssistType::Feedback | ModuleAssistType::InActivity => {
                                                                if let Some(text) = &module_assist.text {
                                                                    dom.prop("body", text)
                                                                } else {
                                                                    dom
                                                                }
                                                            }
                                                        }
                                                    }))
                                                    .event(clone!(state => move |_evt: events::Close| {
                                                        state.show_assist(false);
                                                    }))
                                                    .apply_if(module_assist.audio.is_some(), clone!(state, module_assist => move |dom| {
                                                        if module_assist.is_audio_only() {
                                                            dom
                                                        } else {
                                                            let hover = Mutable::new(false);
                                                            dom.child(html!("button-rect", {
                                                                .prop("slot", "actions")
                                                                .prop("kind", "text")
                                                                .prop("color", "lightBlue")
                                                                .prop("size", "regular")
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
                                                                    state.play_assist_audio();
                                                                }))
                                                            }))
                                                        }
                                                    }))
                                                    .apply(clone!(state, module_assist => move |dom| {
                                                        if module_assist.is_audio_only() {
                                                            dom
                                                        } else {
                                                            let hover = Mutable::new(false);
                                                            dom.child(html!("button-rect", {
                                                                .prop("slot", "actions")
                                                                .prop("kind", "filled")
                                                                .prop("color", "blue")
                                                                .prop("size", "regular")
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
                                                                    state.show_assist(false);
                                                                }))
                                                            }))
                                                        }
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
                        state.toggle_paused();
                    }))
                }),
                html!("jig-play-move-button", {
                    .prop("slot", "back")
                    .prop("kind", "back")
                    .visible_signal(state.jig_and_active_module_signal().map(|(jig, active_module)| {
                        // if module already loaded and not first module
                        match (jig, active_module) {
                            (Some(_jig), Some(active_module)) => {
                                active_module != 0
                            },
                            _ => false,
                        }
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.navigate_back_or_handle();
                    }))
                    .global_event(clone!(state => move |e: events::KeyUp| {
                        state.navigate_from_keyboard_event(KeyEvent::from(e));
                    }))
                }),
                html!("jig-play-progress-bar", {
                    .prop("slot", "progress")
                    .prop_signal("percent", state.progress_signal())
                }),
                html!("jig-play-move-button", {
                    .prop("slot", "forward")
                    .prop("kind", "forward")
                    .event(clone!(state => move |_: events::Click| {
                        state.navigate_forward_or_handle();
                    }))
                    .global_event(clone!(state => move |e: events::KeyUp| {
                        state.navigate_from_keyboard_event(KeyEvent::from(e));
                    }))
                }),
            ])
            .child_signal(state.render_time_indicator())
            .child_signal(state.render_done_popup())
            .child_signal(state.render_time_up_popup())
            .child_signal(state.play_login_popup_shown.signal().map(move|play_login_popup_shown| {
                match play_login_popup_shown {
                    false => None,
                    true => {
                        Some(dialog! {
                            .prop("slot", "dialog")
                            .child(html!("home-login-before-play", {
                                .apply_if(is_iframe(), clone!(state => move |dom| {
                                    dom.child(html!("fa-button", {
                                        .prop("slot", "close")
                                        .prop("icon", "fa-solid fa-xmark")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.close_player();
                                        }))
                                    }))
                                }))
                            }))
                        })
                    },
                }
            }))
        })
    }

    fn active_module_has_scoring(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let active_module = self.active_module.signal(),
            let jig = self.jig.signal_cloned() => move {
                match (active_module, jig) {
                    (Some(active_module), Some(jig)) if jig.jig_data.modules[*active_module].kind.has_scoring() => true,
                    _ => false,
                }
            }
        }
    }

    /// Emits `true` if the module doesn't exist in the list of modules or if the jig is `None`.
    /// Otherwise emits the value of the module's `is_complete` field.
    fn active_module_valid_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        self.jig_and_active_module_signal()
            .map(|(jig, active_module_index)| {
                match (jig, active_module_index) {
                    (Some(jig), Some(active_module_index)) => {
                        match &jig.jig_data.modules.get(active_module_index) {
                            Some(module) => {
                                module.is_complete || matches!(module.kind, ModuleKind::Legacy)
                            }
                            None => true, // Active module isn't in the list
                        }
                    }
                    _ => true, // Jig isn't set
                }
            })
    }

    fn jig_and_active_module_signal(
        self: &Rc<Self>,
    ) -> impl Signal<Item = (Option<JigResponse>, Option<usize>)> {
        map_ref! {
            let jig = self.jig.signal_cloned(),
            let active_module = self.active_module.signal_cloned() => (
                jig.clone(), *active_module
            )
        }
    }

    fn ten_sec_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        self.timer
            .signal_cloned()
            .map(|timer| {
                DefaultSignal::new(
                    false,
                    timer.map(|timer| timer.time.signal().map(|time| time == 10)),
                )
            })
            .flatten()
    }

    fn progress_signal(self: &Rc<Self>) -> impl Signal<Item = u32> {
        (map_ref! {
            let active_module = self.active_module.signal(),
            let jig = self.jig.signal_cloned() =>
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

    fn render_done_popup(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        state.done.signal().map(clone!(state => move |done| {
            match done {
                false => None,
                true => {
                    state.set_timer_paused(true);
                    Some(html!("dialog-overlay", {
                        .prop("slot", "dialog")
                        .prop("open", true)
                        .prop("autoClose", false)
                        .child(html!("jig-play-done-popup", {
                            .apply(|mut dom| {
                                if state.scoring.get() {
                                    dom = dom.prop_signal("score", state.points.signal().map(|p| p * 100));
                                };
                                if !state.scoring.get() {
                                    dom = dom.child(
                                        html!("jig-play-done-action", {
                                            .prop("slot", "actions")
                                            .prop("kind", "replay")
                                            .event(clone!(state => move |_: events::Click| {
                                                state.navigate_to_index(
                                                    0
                                                );
                                            }))
                                        })
                                    );
                                }
                                if !state.is_student {
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
                                dom = dom.child(
                                    html!("jig-play-done-action", {
                                        .prop("slot", "actions")
                                        .prop("kind", "exit")
                                        .text("exit")
                                        .event(clone!(state => move |_: events::Click| {
                                            if is_iframe() {
                                                state.close_player();
                                            } else {
                                                Route::Home(HomeRoute::Home).redirect();
                                            }
                                        }))
                                    })
                                );
                                dom
                            })
                        }))
                    }))
                },
            }
        }))
    }

    fn time_up_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        let timer_signal = self
            .timer
            .signal_cloned()
            .map(|timer| {
                DefaultSignal::new(
                    false,
                    timer.map(|timer| timer.time.signal().map(|time| time == 0)),
                )
            })
            .flatten();
        map_ref! {
            let timer = timer_signal,
            let done = self.done.signal() => *timer && !*done
        }
    }

    fn render_time_up_popup(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        state.time_up_signal().map(clone!(state => move |time_up| {
            match time_up {
                false => None,
                true => {
                    Some(html!("dialog-overlay", {
                        .prop("slot", "dialog")
                        .prop("open", true)
                        .prop("autoClose", false)
                        .child(html!("jig-play-time-up-popup", {
                            .apply(|mut dom| {
                                if state.scoring.get() {
                                    dom = dom.child(
                                        html!("jig-play-done-action", {
                                            .prop("slot", "actions")
                                            .prop("kind", "continue")
                                            .event(clone!(state => move |_: events::Click| {
                                                state.navigate_forward_or_handle();
                                            }))
                                        })
                                    );
                                };
                                if !state.scoring.get() {
                                    dom = dom.child(
                                        html!("jig-play-done-action", {
                                            .prop("slot", "actions")
                                            .prop("kind", "replay")
                                            .event(clone!(state => move |_: events::Click| {
                                                // Clear the assist so that they don't play/show once the
                                                // activity is reloaded.
                                                state.set_module_assist(None);
                                                state.reload_iframe();
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

    fn render_time_indicator(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        self.timer.signal_cloned().map(clone!(state => move |timer| {
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
                            .future(state.ten_sec_signal().for_each(move |less_than_10_sec| {
                                if less_than_10_sec {
                                    let buzz_method = Reflect::get(
                                        &elem,
                                        &JsValue::from_str("buzz")
                                    )
                                        .unwrap_ji();
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

    // fn fullscreen_enabled() -> bool {
    //     // once webkit drops the prefix, we can just use `document().fullscreen_enabled()`

    //     let webkit_fullscreen_enabled = js_sys::Reflect::get(
    //         &document(),
    //         &JsValue::from_str("webkitFullscreenEnabled")
    //     );
    //     match webkit_fullscreen_enabled {
    //         Ok(webkit_fullscreen_enabled) => {
    //             webkit_fullscreen_enabled.as_bool().unwrap_or_default()
    //         },
    //         Err(_) => {
    //             document().fullscreen_enabled()
    //         },
    //     }
    // }
}
