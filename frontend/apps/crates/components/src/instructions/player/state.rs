use dominator::{Dom, html, clone};
use dominator::animation::MutableAnimation;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;
use shared::domain::jig::module::body::Instructions;
use crate::animation::fade::*;
use std::cell::RefCell;
use web_sys::AudioContext;
use crate::audio_mixer::{AudioMixer, AudioHandle};

pub struct InstructionsPlayer {
    pub(super) data: Instructions,
    pub(super) fade: Fade,
    pub(super) audio: RefCell<Option<AudioHandle>>
}

impl InstructionsPlayer {
    pub fn new(data:Instructions) -> Self {
        /*
        let data = Instructions {
            text: Some("instructions here!".to_string()),
            audio: None
        };
        */
        let animation = MutableAnimation::new(1000.0);
        Self {
            data,
            fade: Fade::new(
                FadeKind::Out,
                1000.0,
                true,
                Some(3000.0),
                None::<fn()>
            ),
            audio: RefCell::new(None),
        }
    }
}
