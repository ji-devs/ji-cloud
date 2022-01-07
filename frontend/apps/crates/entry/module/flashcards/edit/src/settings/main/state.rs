use crate::state::*;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::{_groups::cards::Card, flashcards::DisplayMode};
use std::rc::Rc;
use utils::prelude::*;

pub struct MainSettings {
    pub base: Rc<Base>,
    pub display_mode: Mutable<DisplayMode>,
    pub left: Card,
    pub right: Card,
}

impl MainSettings {
    pub fn new(base: Rc<Base>) -> Self {
        let settings = &base.extra.settings;

        let (left, right) = {
            let pairs = base.pairs.lock_ref();
            let pair = pairs.get(0).unwrap_ji();
            (pair.0.clone().into(), pair.1.clone().into())
        };

        let display_mode = settings.display_mode.clone();

        Self {
            base,
            left,
            right,
            display_mode,
        }
    }
}
