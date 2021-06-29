use futures_signals::signal_vec::MutableVec;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::{Jig, LiteModule, JigId, module::ModuleId, ModuleKind};
use std::rc::Rc;
use crate::edit::sidebar::module::state::State as ModuleState;
use utils::drag::Drag;

//Instead of getting the anchor from the mouse point
//These values keep it at a consistent, predesiged place
const ANCHOR_X:f64 = 20.0;
const ANCHOR_Y:f64 = 100.0;

pub struct State {
    pub module: Rc<ModuleState>,
    pub inner: Drag,
}

impl State {
    pub fn new(module: Rc<ModuleState>, x: i32, y: i32) -> Self {
        Self {
            module,
            inner: Drag::new(x, y, ANCHOR_X, ANCHOR_Y, false),
        }
    }

}
