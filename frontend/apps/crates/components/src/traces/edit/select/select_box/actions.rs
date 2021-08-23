use utils::{prelude::*, drag::*, resize::{ResizeInfo, get_resize_info}, math::BoundsF64};
use shared::domain::jig::module::body::Transform;
use super::state::*;
use std::rc::Rc;

impl SelectBox {
    pub fn reset_bounds(&self, resize_info:&ResizeInfo) {
        self.bounds.set(
            self.elem.borrow()
                .as_ref()
                .map(|elem| {
                    let rect = elem.get_bounding_client_rect();
                    BoundsF64::new_from_dom_normalized(&rect, &resize_info)
                })
        );
    }

    pub fn start_drag(&self, x: i32, y: i32) {
        self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);
    
                self.transform_override.replace_with(|t| {
                    let mut t = t.clone();
                    t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                    t
                });
            }
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) -> Option<Transform> {
        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
            Some(self.transform_override.get_cloned())
        } else {
            None
        }
    }
}
