use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform};
use crate::transform::state::TransformState;
use dominator::clone;
use super::{
    state::*,
    menu::state::*,
    trace::state::*,
};
use crate::traces::utils::TraceExt;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};


impl Draw {
    /// 
    pub fn start_draw(&self, x: i32, y: i32) {

        self.trace.transform.reset();

        let resize_info = get_resize_info();
        let (pos_x, pos_y) = resize_info.get_pos_px(x as f64, y as f64);

        self.drag.set(Some(Drag::new(pos_x as i32, pos_y as i32, 0.0, 0.0)));

        let (norm_x, norm_y) = resize_info.get_pos_normalized(x as f64, y as f64);

        self.draw_points.set(vec![(norm_x, norm_y)]);
        self.menu.set(None);

        self.display_trace.set_neq(false);

    }
    pub fn end_draw(&self, x: i32, y: i32) {

        if let Some(drag) = self.drag.replace(None) {

            self.propogate_to_trace(true);
            
            if let Some(_) = self.trace.calc_bounds(false) {

                self.display_trace.set_neq(true);
                self.recreate_deco();
            }
        }
    }
    pub fn move_draw(&self, x: i32, y: i32) {

        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some(_) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (norm_x, norm_y) = resize_info.get_pos_normalized(x as f64, y as f64);
                self.draw_points.lock_mut().push((norm_x, norm_y));
            }
        }
    }

    pub fn recreate_deco(&self) {
        if let Some(bounds) = self.trace.calc_bounds(true) {
            let resize_info = get_resize_info();
            let (width, height) = resize_info.get_size_full(bounds.width, bounds.height);


            self.trace.transform.size.set(Some((width, height)));
            self.menu.set(Some(Menu::new(self.trace.clone())));
        }
    }

    pub fn propogate_to_trace(&self, set_translation: bool) {
        *self.trace.shape.lock_mut() = TraceShape::new_path(self.draw_points.get_cloned());
        if let Some(bounds) = self.trace.calc_bounds(false) {
            match &*self.trace.shape.lock_ref() {
                TraceShape::Path(path) => {
                    let path:&mut Vec<(f64, f64)> = &mut *path.lock_mut();

                    for (x,y) in path {
                        *x -= bounds.x;
                        *y -= bounds.y;
                    }
                },
                _ => {}
            }
            if set_translation {
                self.trace.transform.get_inner_mutable().lock_mut().set_translation_2d(bounds.x, bounds.y);
            }
        }
    }



    pub fn shape_free(&self) {
        self.propogate_to_trace(false);
        self.recreate_deco();
    }

    pub fn shape_rect(&self) {
        self.propogate_to_trace(false);

        if let Some(bounds) = self.trace.calc_bounds(true) {
            *self.trace.shape.lock_mut() = TraceShape::Rect(bounds.width, bounds.height);
            self.recreate_deco();
        }

    }
    pub fn shape_ellipse(&self) {

        self.propogate_to_trace(false);

        if let Some(bounds) = self.trace.calc_bounds(true) {
            let radius_x = bounds.width / 2.0;
            let radius_y = bounds.height/ 2.0;
            let x = bounds.x;
            let y = bounds.y;

            *self.trace.shape.lock_mut() = TraceShape::Ellipse(radius_x, radius_y);
            self.trace.transform.get_inner_mutable().lock_mut().set_translation_2d(x, y);

            self.recreate_deco();
        }

    }
    pub fn done(&self) {
        if let Some(bounds) = self.trace.calc_bounds(true) {
            (self.on_finished) (Some(self.trace.to_raw()));
        } else {
            (self.on_finished) (None);
        }

    }
}
