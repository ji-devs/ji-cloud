use crate::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use components::module::_groups::cards::lookup::Side;
use rand::prelude::*;
use shared::domain::jig::module::body::card_quiz::ModuleData as RawData;
use futures_signals::signal::Mutable;

pub struct Settings {
    pub rng: Rc<RefCell<ThreadRng>>,
    pub top_side: Mutable<Side>,
    pub n_choices: Mutable<usize>
}

impl Settings {
    pub fn new(raw:&RawData) -> Self {
        Self {
            rng: Rc::new(RefCell::new(thread_rng())),
            top_side: Mutable::new(Side::Left),
            n_choices: Mutable::new(3),
        }
    }
}
