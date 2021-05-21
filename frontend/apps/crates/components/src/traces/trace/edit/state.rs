use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform};
use crate::transform::state::TransformState;
use dominator::clone;
use super::{
    super::state::Trace
};
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub struct Edit {
    pub index: Option<usize>,
    pub trace: Trace,
    pub drag: Mutable<Option<Drag>>,
}

impl Edit {
    pub fn new(index: Option<(usize, Trace)>, on_change_cb: Rc<Box<dyn Fn()>>) -> Self {

        let (index, trace) = match index {
            Some((index, trace)) => (Some(index), trace),
            None => (None, Trace::new(None, on_change_cb))
        };

        Self {
            index,
            trace,
            drag: Mutable::new(None),
        }
    }

}
