use std::{rc::Rc, cell::RefCell};
use crate::base::game::state::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, Mutable}
};
use components::traces::{
    utils::TraceExt,
    bubble::state::TraceBubble,
};
use shared::domain::jig::module::body::{
    Audio,
    _groups::design::Trace,
    drag_drop::{Next, DragDropTrace}
};
use web_sys::AudioContext;
use std::collections::HashSet;

pub struct PlayState {
    pub game: Rc<Game>,
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        Rc::new(Self {
            game,
        })
    }
}
