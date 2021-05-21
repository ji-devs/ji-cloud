use futures_signals::{
    map_ref,
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform, TraceShape};
use crate::transform::state::TransformState;
use dominator::clone;
use utils::prelude::*;

#[derive(Clone)]
pub struct Trace {
    pub transform: Rc<TransformState>,
    pub shape: Mutable<TraceShape>,
}


impl Trace {
    pub fn new(raw: Option<RawTrace>, on_change_cb: Rc<Box<dyn Fn()>>) -> Self {
        let raw = match raw {
            Some(raw) => raw,
            None => {
                RawTrace {
                    transform: Transform::identity(),
                    shape: TraceShape::Path(Vec::new()) 
                }
            }
        };

        Self {
            transform: Rc::new(TransformState::new(
                raw.transform, 
                None, 
                Some(move |_| {
                    on_change_cb();
                })
            )),
            shape: Mutable::new(raw.shape) 
        }
    }

    pub fn to_raw(&self) -> RawTrace {
        RawTrace {
            transform: self.transform.get_inner_clone(),
            shape: self.shape.get_cloned()
        }
    }

}

