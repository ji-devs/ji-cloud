use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, TraceShape, Transform};
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

        self.trace.shape.set(TraceShape::Path(vec![(norm_x, norm_y)]));
    }
    pub fn end_draw(&self, x: i32, y: i32) {

        if let Some(drag) = self.drag.replace(None) {
            log::info!("TODO - calc bounds (for showing menu and pushing final SVG!");
        }
    }
    pub fn move_draw(&self, x: i32, y: i32) {

        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some(_) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (norm_x, norm_y) = resize_info.get_pos_normalized(x as f64, y as f64);
                let mut shape = self.trace.shape.lock_mut();
                match &mut *shape {
                    TraceShape::Path(path) => {
                        path.push((norm_x, norm_y));
                    }
                    _ => {
                        unimplemented!("don't know how to handle other shapes yet!!")
                    }

                }
            }
        }
    }
}
