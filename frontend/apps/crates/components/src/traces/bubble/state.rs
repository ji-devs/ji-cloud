use utils::{prelude::*, math::bounds::BoundsF64};
use shared::domain::jig::module::body::Audio;
use crate::animation::fade::*;
use crate::audio_player::AudioPlayer;
use std::cell::RefCell;

pub struct TraceBubble {
    pub bounds: BoundsF64,
    pub audio: Option<Audio>,
    pub text: Option<String>,
    pub fade: Fade,
    pub(super) audio_player_instance: RefCell<Option<AudioPlayer>>

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
}
