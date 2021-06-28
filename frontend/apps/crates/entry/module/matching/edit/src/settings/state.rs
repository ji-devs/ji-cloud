use crate::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;
use shared::domain::jig::module::body::matching::ModuleData as RawData;
use futures_signals::signal::Mutable;
use components::module::_groups::cards::lookup::Side;

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
