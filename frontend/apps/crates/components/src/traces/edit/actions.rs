use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, PathPoint, Transform};
use crate::transform::state::TransformState;
use dominator::clone;
use super::state::*;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};

impl Edit {
    pub fn start_draw(&self, x: i32, y: i32) {

        let resize_info = get_resize_info();
        let (pos_x, pos_y) = resize_info.get_pos_px(x as f64, y as f64);

        self.drag.set(Some(Drag::new(pos_x as i32, pos_y as i32, 0.0, 0.0)));

        let (norm_x, norm_y) = resize_info.get_pos_normalized(x as f64, y as f64);

        self.trace.path.lock_mut().push_cloned(
            PathPoint::MoveTo(norm_x, norm_y)
        );
    }
    pub fn end_draw(&self, x: i32, y: i32) {

        if let Some(drag) = self.drag.replace(None) {
        }
    }
    pub fn move_draw(&self, x: i32, y: i32) {

        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some(_) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (pos_x, pos_y) = resize_info.get_pos_normalized(x as f64, y as f64);
                self.trace.path.lock_mut().push_cloned(
                    PathPoint::LineTo(pos_x, pos_y)
                );
            }
        }
    }
}
