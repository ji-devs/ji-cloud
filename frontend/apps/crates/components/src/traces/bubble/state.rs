use utils::{prelude::*, math::bounds::BoundsF64};
use shared::domain::jig::module::body::Audio;
use crate::animation::fade::*;
use crate::audio_mixer::AudioInstance;
use std::cell::RefCell;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use dominator::clone;

pub struct TraceBubble {
    pub bounds: BoundsF64,
    pub audio: Option<Audio>,
    pub text: Option<String>,
    pub fade: Fade,
    pub(super) audio_player_instance: RefCell<Option<AudioInstance>>

}

impl TraceBubble {
    pub fn new(bounds: BoundsF64, audio: Option<Audio>, text: Option<String>, on_fade_end: Option<impl Fn() + 'static>) -> Self {
        Self {
            bounds,
            audio,
            text,
            audio_player_instance: RefCell::new(None),
            fade: Fade::new(
                FadeKind::Out,
                6_000.0,
                true,
                None,
                on_fade_end
            )
        }
    }

    //Will manage its own lifetime by way of a specific Mutable type
    pub fn set_unset_mutable(bounds: BoundsF64, audio: Option<Audio>, text: Option<String>, mutable: Mutable<Option<Rc<TraceBubble>>>) {

        let instance = Rc::new(TraceBubble::new(
            bounds, 
            audio, 
            text,
            Some(clone!(mutable => move || {
                mutable.set(None)
            }))
        ));

        mutable.set(Some(instance));
    }
}
