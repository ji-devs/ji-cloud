use crate::base::state::*;
use std::{cell::RefCell, rc::Rc};

use components::audio::mixer::AudioHandle;
use futures_signals::signal::Mutable;
use shared::domain::module::body::drag_drop::*;

pub struct Game {
    pub base: Rc<Base>,
    pub phase: Mutable<Phase>,
    pub interactive_audio_handle: Rc<RefCell<Option<AudioHandle>>>,
}

impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let phase = Mutable::new(match base.settings.hint {
            Hint::Highlight => Phase::ShowHints,
            Hint::None => Phase::Playing,
        });

        Rc::new(Self {
            base,
            phase,
            interactive_audio_handle: Default::default(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    ShowHints,
    Playing,
}
