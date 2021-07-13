use components::transform::state::Action;
use utils::{prelude::*, drag::*, resize::get_resize_info};
use super::state::*;
use std::rc::Rc;

impl DragItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        let init = self.item.sticker.get_translation_2d();
        self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.item.get_interactive_unchecked()
                    .target_offset.replace_with(|(acc_x, acc_y)| {
                        (*acc_x - diff_x, *acc_y - diff_y)
                    });
            }
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) {
        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
           
            self.base.set_drag_item_target_offset(self.index, self.get_offset_mutable().get_cloned());


        }
    }
}
