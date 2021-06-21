use std::rc::Rc;
use dominator_helpers::events::Message;
use futures_signals::map_ref;
use utils::{
    iframe::{IframeAction, JigToModuleMessage, ModuleToJigMessage},
    prelude::SETTINGS,
    routes::{ModuleRoute, Route},
};
use futures_signals::signal::{SignalExt, Signal};
use dominator::{Dom, clone, events, html};
use web_sys::HtmlIFrameElement;
use super::{actions, sidebar};

use super::state::State;


pub fn render(state: Rc<State>) -> Dom {

    actions::load_jig(state.clone());

    html!("jig-play-landing", {
        .global_event(clone!(state => move |evt:Message| {
            match evt.try_serde_data::<IframeAction<ModuleToJigMessage>>() {
                Err(m) => log::info!("hmmm got other iframe message: {:?}", m),
                Ok(m) => {
                    actions::on_iframe_message(Rc::clone(&state), m.data)
                },
            };
        }))
        .apply(|dom| {
            if state.is_teacher {
                let sidebar_state = Rc::new(sidebar::state::State::new(state.clone()));
                dom.child(sidebar::dom::render(sidebar_state))
            } else {
                dom
            }
        })
        .children(&mut [
            html!("iframe" => HtmlIFrameElement, {
                .property("slot", "iframe")
                .property_signal("src", state.active_module.signal_cloned().map(clone!(state => move|active_module_index| {
                    match &*state.jig.lock_ref() {
                        None => String::new(),
                        Some(jig) => {
                            let active_module = &jig.modules[active_module_index];

                            let route: String = Route::Module(ModuleRoute::Play(
                                active_module.kind,
                                state.jig_id,
                                active_module.id
                            )).into();
                            let url = unsafe {
                                SETTINGS.get_unchecked()
                                    .remote_target
                                    .spa_iframe(&route)
                            };
                            url
                        },
                    }
                })))
                .after_inserted(clone!(state => move|element| {
                    *state.iframe.borrow_mut() = Some(element);
                }))
            }),
            html!("jig-play-play-button", {
                .property("slot", "play-button")
            }),
            html!("jig-play-play-pause", {
                .property("slot", "play-pause-button")
                .property("mode", "play")
                .event(clone!(state => move |_:events::Click| {
                    actions::sent_iframe_message(Rc::clone(&state), JigToModuleMessage::TimerDone);
                }))
            }),
            html!("jig-play-replay", {
                .property("slot", "replay-background")
            }),
            html!("jig-play-background-music", {
                .property("slot", "replay-background")
            }),
            html!("empty-fragment", {
                .property("slot", "indicators")
                .child_signal(state.timer.signal_cloned().map(|timer| {
                    match timer {
                        None => None,
                        Some(timer) => {
                            Some(html!("jig-play-timer-indicator", {
                                .property_signal("value", timer.time.signal())
                            }))
                        }
                    }
                }))
            }),
            html!("jig-play-points-indicator", {
                .property("slot", "indicators")
                .property_signal("value", state.points.signal())
            }),
            html!("jig-play-move-button", {
                .property("slot", "progress")
                .property("kind", "back")
                .event(clone!(state => move |_: events::Click| {
                    let mut active_module = state.active_module.lock_mut();
                    if *active_module != 0 {
                        *active_module -= 1;
                    }
                }))
            }),
            html!("jig-play-progress-bar", {
                .property("slot", "progress")
                .property_signal("percent", progress_signal(state.clone()))
            }),
            html!("jig-play-move-button", {
                .property("slot", "progress")
                .property("kind", "forward")
                .event(clone!(state => move |_: events::Click| {
                    let mut active_module = state.active_module.lock_mut();
                    if let Some(jig) = &*state.jig.lock_ref() {
                        if *active_module < jig.modules.len() - 1 {
                            *active_module += 1;
                        }
                    }
                }))
            }),
        ])
    })
}


fn progress_signal(state: Rc<State>) -> impl Signal<Item = u32> {
    (map_ref! {
        let active_module = state.active_module.signal(),
        let jig = state.jig.signal_cloned() =>
            (*active_module, jig.clone())
    }).map(move|(active_module_index, jig)| {
        match jig {
            None => 0,
            Some(jig) => {
                let len = jig.modules.len();
                let step_percent = 100f32 / len as f32;
                let current_progress = active_module_index as f32 * step_percent;
                // TODO: ask corrine if this should be here
                let current_progress = current_progress + step_percent;
                log::info!("{}", current_progress);
                current_progress.round() as u32
            },
        }
    })
}
