use crate::base::state::*;
use std::rc::Rc;

use futures_signals::signal::Mutable;

pub struct Game {
    pub base: Rc<Base>,
    pub phase: Mutable<Phase>,
}

impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        // let phase = Mutable::new(match base.settings.hint {
        //     Hint::Highlight => Phase::ShowHints,
        //     Hint::None => Phase::Playing,
        // });
        // TODO find-answer

        Rc::new(Self {
            base,
            phase: Mutable::new(Phase::Playing),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Playing,
}
