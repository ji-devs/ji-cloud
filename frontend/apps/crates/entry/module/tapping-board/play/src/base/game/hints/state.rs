use crate::base::state::*;
use std::rc::Rc;
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::{Mutable, Signal, SignalExt}
};
use shared::domain::jig::module::body::tapping_board::*;
use super::super::state::*;

pub struct Hints {
    pub game: Rc<Game>,
}

impl Hints {
    pub fn new(game: Rc<Game>) -> Rc<Self> {

        Rc::new(Self {
            game,
        })

    }
}
