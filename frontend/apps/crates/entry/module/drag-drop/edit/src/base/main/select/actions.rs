use components::transform::state::Action;
use utils::{prelude::*, drag::*, resize::get_resize_info};
use super::state::*;
use std::rc::Rc;

impl SelectItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        self.item.sticker.transform().start_tracking_action(Action::Move, x, y);
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        self.item.sticker.transform().mouse_move(x, y);

    }

    pub fn try_end_drag(&self, x: i32, y: i32) {
        self.item.sticker.transform().stop_tracking_action(x, y);
    }
}
