use components::transform::state::Action;
use utils::{prelude::*, drag::*, resize::get_resize_info};
use super::state::*;
use std::rc::Rc;

impl DragItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        
        let data = self.item.get_interactive_unchecked();

        if data.target_transform.lock_ref().is_none() {
            data.target_transform.set(Some(self.item.sticker.transform().get_inner_clone()));
        }

        self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.item.get_interactive_unchecked()
                    .target_transform.replace_with(|t| {
                        t.as_ref().map(|t| {
                            let mut t = t.clone();
                            t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                            t
                        })
                    });
            }
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) {
        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
         
            if let Some(transform) = self.item.get_interactive_unchecked().target_transform.get_cloned() {
                self.base.set_drag_item_target_transform(self.index, transform);
            }


        }
    }
}
