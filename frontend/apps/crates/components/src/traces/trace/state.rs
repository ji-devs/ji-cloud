use futures_signals::{
    map_ref,
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform, PathPoint};
use crate::transform::state::TransformState;
use dominator::clone;
use utils::prelude::*;

#[derive(Clone)]
pub struct Trace {
    pub transform: Rc<TransformState>,
    pub path: Rc<MutableVec<PathPoint>>,
}


impl Trace {
    pub fn new(raw: Option<RawTrace>, on_change_cb: Rc<Box<dyn Fn()>>) -> Self {
        let raw = match raw {
            Some(raw) => raw,
            None => {
                RawTrace {
                    transform: Transform::identity(),
                    path: Vec::new(),
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
            path: Rc::new(MutableVec::new_with_values(raw.path)),
        }
    }

    pub fn to_raw(&self) -> RawTrace {
        RawTrace {
            transform: self.transform.get_inner_clone(),
            path: self.path.lock_ref().to_vec()
        }
    }

}

