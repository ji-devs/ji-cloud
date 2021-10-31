use super::state::Base;
use utils::{prelude::*, resize::get_resize_info};
use std::sync::atomic::Ordering;

#[derive(Clone, Debug)]
pub struct StageClick {
    pub mouse_x: f64,
    pub mouse_y: f64,
}

impl StageClick {
    pub fn to_normalized(&self) -> (f64, f64) {
        get_resize_info()
            .get_pos_normalized(self.mouse_x, self.mouse_y)
    }
}
impl Base {
    pub fn on_click(&self, mouse_x: f64, mouse_y: f64) {
        if self.stage_click_allowed.load(Ordering::SeqCst) {
            let stage_click = StageClick {mouse_x, mouse_y};
            for f in self.stage_click_listeners.borrow_mut().iter_mut() {
                f(stage_click.clone());
            }
        }
    }

    pub fn allow_stage_click(&self) {
        self.stage_click_allowed.store(true, Ordering::SeqCst);
    }
}