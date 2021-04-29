use super::state::*;
use utils::drag::Drag;
use std::rc::Rc;
use utils::{prelude::*, resize::get_resize_info};

impl TransformState {
    pub fn start_tracking_action(&self, action: Action, x: i32, y:i32) {
        *self.action.borrow_mut() = Some(action);

        let resize_info = get_resize_info();
        let (screen_x, screen_y) = resize_info.get_pos_px(x as f64, y as f64);

        let (tx, ty) = self.transform.lock_ref().get_translation_2d();
        let (tx, ty) = resize_info.get_pos_denormalized(tx, ty);

        let anchor_x = screen_x - tx;
        let anchor_y = screen_y - ty;

        self.drag.set(Some(Drag::new(x, y, anchor_x, anchor_y)));
    }

    pub fn mouse_move(&self, x: i32, y:i32) {
        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some((pos, diff)) = drag.update(x, y) {
                if let Some(action) = self.action.borrow().as_ref() {
                    match action {
                        Action::Move => {
                            let resize_info = get_resize_info();
                            let (pos_x, pos_y) = resize_info.get_pos_normalized(pos.x as f64, pos.y as f64);
                            let mut transform = self.transform.lock_mut();
                            transform.set_translation_2d(pos_x, pos_y);

                            /*
                            let (diff_x, diff_y) = resize_info.get_pos_normalized(diff.x as f64, diff.y as f64);

                            log::info!("{}, {} -> {}, {}", diff.x, diff.y, diff_x, diff_y);
                            let mut transform = self.transform.lock_mut();

                            let (prev_x, prev_y) = transform.get_translation_2d();

                            transform.set_translation_2d(prev_x + diff_x, prev_y + diff_y);
                            */
                            //self.transform.lock_mut().set_translation_2d(pos.x as f64, pos.y as f64);

                        }
                    }
                }
                
            }
        }
    }

    pub fn mouse_up(&self, x: i32, y:i32) {
        if let Some(drag) = self.drag.replace(None) {
            
        }
    }
}

