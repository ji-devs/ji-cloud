use std::rc::Rc;

use discard::Discard;
use futures_signals::signal::{Mutable, SignalExt};

use dominator::{clone, events, html, Dom};

use utils::{events::ModuleResizeEvent, iframe::*, keyboard::KeyEvent, prelude::*, resize::*};

use super::{ending::*, loading::dom::render_loading, state::*};
use crate::{
    audio::mixer::AUDIO_MIXER, module::_common::play::prelude::*,
    overlay::container::OverlayContainer,
};
use shared::domain::module::body::{BodyExt, InstructionsType, ModeExt, StepExt};

pub fn render_page_body<RawData, Mode, Step, Base>(
    state: Rc<GenericState<RawData, Mode, Step, Base>>,
) where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    let sig =
            state.phase.signal_cloned().map(clone!(state => move |phase| {
                let page_kind = match phase.as_ref() {
                    InitPhase::Loading(_) | InitPhase::WaitingIframeRaw(_) => ModulePageKind::GridPlain,
                    InitPhase::Ready(_) => ModulePageKind::Iframe
                };

                let has_resized_once = Mutable::new(!page_kind.is_resize());

                html!(page_kind.element_name(), {
                        .global_event(move |e: events::KeyUp| {
                            // We only want to handle navigating between activities using the arrow keys at this stage, so
                            // first check that this event is from a movement key. Otherwise we'll potentially spam the parent
                            // with keyboard events.
                            let key_event = KeyEvent::from(e);
                            if key_event.key.is_move_key() {
                                let msg = IframeAction::new(ModuleToJigPlayerMessage::KeyEvent(key_event));
                                let _ = msg.try_post_message_to_player();
                            }
                        })
                        .apply_if(page_kind.add_scrollable_attribute(), |dom| {
                            dom.prop("scrollable", true)
                        })
                        .apply_if(RawData::is_legacy(), |dom| {
                            dom.prop("legacy", true)
                        })
                        .event(clone!(has_resized_once => move |event:ModuleResizeEvent| {
                            //in utils / global static
                            set_resize_info(event.data());
                            has_resized_once.set_neq(true);
                        }))
                        .children_signal_vec({
                            has_resized_once.signal()
                                .map(clone!(state, phase => move |has_resized_once| {
                                    if !has_resized_once {
                                        vec![]
                                    } else {
                                        match phase.as_ref() {
                                            InitPhase::Loading(_) => {
                                                vec![render_loading(state.clone())]
                                            },
                                            InitPhase::WaitingIframeRaw(on_raw) => {
                                                vec![render_iframe_wait_raw(state.clone(), on_raw.clone())]
                                            },
                                            InitPhase::Ready(ready) => {
                                                vec![render_player(state.clone(), ready.base.clone(), ready.jig_player)]
                                            },
                                        }
                                    }
                                }))
                                .to_signal_vec()
                        })
                        .child(OverlayContainer::new().render(Some("overlay")))
                })
            }));

    state
        .page_body_switcher
        .load(sig.for_each(clone!(state => move |dom| {
            {
                // Discard the previous body and set the current handle to None.
                // This forces dominator to release all references held by this handle.
                let current_handle = state.dom_body_handle.replace(None);
                if let Some(current_handle) = current_handle {
                    current_handle.discard();
                }
            }

            // Append the new body and set the handle.
            let handle = dominator::append_dom(&dominator::get_id("root"), dom);
            state.dom_body_handle.set(Some(handle));
            async move {}
        })));
}

//This is just a placeholder to get messages
//It'll be replaced when the iframe data arrives
fn render_iframe_wait_raw<RawData, Mode, Step, Base>(
    _state: Rc<GenericState<RawData, Mode, Step, Base>>,
    on_raw: Rc<Box<dyn Fn(RawData)>>,
) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    html!("empty-fragment", {
        .global_event(clone!(on_raw => move |evt:dominator_helpers::events::Message| {
            if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                log::info!("got iframe data!");
                //on_raw was stashed from the original State::new()
                on_raw(msg.data);
            } else {
                log::info!("hmmm got other iframe message...");
            }
        }))
        .after_inserted(|_elem| {
            //On mount - send an empty IframeInit message to let the *parent* know we're ready
            //parent here is probably the editor window (i.e. we've been told to wait for raw data)
            //note that by default try_post_message_to_editor() is by default IframeTarget::Top
            //TODO: determine if this can be changed to try_post_message_to_editor()
            IframeInit::empty()
                .try_post_message_to_parent()
                .unwrap_ji();
        })
    })
}

fn render_player<RawData, Mode, Step, Base>(
    state: Rc<GenericState<RawData, Mode, Step, Base>>,
    base: Rc<Base>,
    jig_player: bool,
) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    let instructions = base.get_instructions();
    let feedback = base.get_feedback();
    let is_screenshot = utils::screenshot::is_screenshot_url();

    html!("empty-fragment", {
        .prop("slot", "main")
        .child(Base::render(base.clone()))
        .future(feedback.signal_cloned().for_each(move |feedback| async move {
            let msg = IframeAction::new(ModuleToJigPlayerMessage::Instructions(feedback.map(|feedback| (feedback, InstructionsType::Feedback))));
            let _ = msg.try_post_message_to_player();
        }))
        .apply_if(jig_player, |dom| {
            dom
                .global_event(clone!(state => move |evt: dominator_helpers::events::Message| {
                    if let Ok(msg) = evt.try_serde_data::<IframeAction<JigToModulePlayerMessage>>() {
                        match msg.data {
                            JigToModulePlayerMessage::Play => {
                            },
                            JigToModulePlayerMessage::Pause => {
                            },
                            JigToModulePlayerMessage::TimerDone => {
                            },
                            JigToModulePlayerMessage::InstructionsDone(instructions_type) => {
                                if let InitPhase::Ready(base) = &*state.phase.get_cloned() {
                                    if let ModulePlayPhase::PreStart = base.base.play_phase().get_cloned() {
                                        // When instructions have completed during the *PreStart* phase, then we move on
                                        // to the Playing phase.
                                        base.base.set_play_phase(ModulePlayPhase::Playing);
                                    } else {
                                        // During subsequent phases, we let the individual activities decide how to handle
                                        // the completion of instructions.
                                        base.base.handle_instructions_ended(instructions_type);
                                    }
                                }
                            },
                        }
                    } else {
                        log::info!("hmmm got other iframe message...");
                    }
                }))
                .after_inserted(|_elem| {
                    //On mount - send an empty IframeInit message to let the player know we're ready
                    IframeInit::empty()
                        .try_post_message_to_player()
                        .unwrap_ji();
                })
        })

        .apply_if(!is_screenshot, |dom| {
            dom.child_signal(base.play_phase().signal().map(clone!(base => move |curr_play_phase| {
                match curr_play_phase {
                    ModulePlayPhase::Preload => {
                        Some(html!("module-preload", {
                        }))
                    },

                    ModulePlayPhase::Init => {
                        Some(html!("module-play-button", {
                            .event(clone!(base => move |_evt:events::Click| {
                                start_playback(base.clone());
                            }))
                            .after_inserted(clone!(state, base => move |_elem| {
                                if AUDIO_MIXER.with(|mixer| mixer.context_available()) || state.opts.skip_play {
                                    start_playback(base);
                                }
                            }))
                        }))
                    },

                    ModulePlayPhase::PreStart => {
                        // Everything gets set up in the JIG player in PreStart phase.
                        // This will mark the activity as started in the player, but the activity itself would
                        // only start playing once it's in the Playing phase.
                        if jig_player {
                            let timer_seconds = base.get_timer_seconds();

                            let msg = IframeAction::new(ModuleToJigPlayerMessage::Start(timer_seconds));

                            match timer_seconds {
                                Some(x) => log::info!("Starting with a {} seconds timer", x),
                                None => log::info!("Starting without a timer")
                            }

                            //let the player know we're starting
                            msg.try_post_message_to_player().unwrap_ji();
                            match &instructions {
                                Some(instructions) if instructions.has_content() => {
                                    let msg = IframeAction::new(ModuleToJigPlayerMessage::Instructions(Some((instructions.clone(), InstructionsType::Instructions))));
                                    let _ = msg.try_post_message_to_player();
                                },
                                _ => {
                                    base.play_phase().set_neq(ModulePlayPhase::Playing);
                                }
                            }
                        }
                        None
                    },

                    ModulePlayPhase::Playing => {
                        // Playing phase will affect how the individual activities render their content.
                        // So nothing to do here.
                        None
                    },
                    ModulePlayPhase::Ending(ending) => {
                        Some(Ending::render(Ending::new(ending)))
                    }
                }
            })))
        })
    })
}

fn start_playback<Base>(base: Rc<Base>)
where
    Base: BaseExt + 'static,
{
    base.play_phase().set_neq(ModulePlayPhase::PreStart);
    Base::play(base);
}
