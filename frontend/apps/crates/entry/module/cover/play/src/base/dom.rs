use super::state::*;
use components::{
    audio::mixer::AUDIO_MIXER, backgrounds::dom::render_backgrounds_raw,
    module::_common::play::prelude::*, stickers::dom::render_stickers_raw,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::cover::Next;
use std::rc::Rc;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .prop("slot", "main")
            .future(state.module_phase.signal_cloned().for_each(clone!(state => move |phase| {
                if let ModulePlayPhase::Playing = phase {
                    *state.audio_handle.borrow_mut() = state.audio.as_ref().map(|audio| {
                        AUDIO_MIXER.with(|mixer| {
                            mixer.play_on_ended(
                                audio.into(),
                                false,
                                clone!(state => move || {
                                    state.audio_handle.borrow_mut().take();
                                    match state.play_settings.next {
                                        Next::Auto | Next::AfterAudio => {
                                            state.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
                                        }
                                        _ => {}
                                    }
                                }),
                            )
                        })
                    });
                }
                async {}
            })))
            .children(&mut [
                render_backgrounds_raw(&state.backgrounds, state.theme_id, None),
                render_stickers_raw(&state.stickers, state.theme_id),
            ])
        })
    }
}
