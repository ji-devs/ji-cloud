use crate::traces::edit::draw::trace::state::*;
use utils::{
    resize::{get_resize_info, ResizeInfo},
    unwrap::UnwrapJiExt,
};

#[derive(Clone)]
pub struct Menu {
    pub trace: DrawTrace,
}

//in normalized units
const MENU_MARGIN: f64 = 0.01;

//in real-world units
const MENU_WIDTH: f64 = 217.0;

impl Menu {
    pub fn new(trace: DrawTrace) -> Self {
        Self { trace }
    }

    pub fn get_pos(&self, _resize_info: &ResizeInfo) -> (f64, f64) {
        let bounds = match self.trace.transform.get_dom_rect_bounds() {
            Some(bounds) => bounds,
            None => {
                //without the real DomRects, do our best with the basic transform bounds
                let mut bounds = self.trace.transform.get_aabb_no_rotation_bounds_px(false);
                bounds.y += 10.0; //dunno why this shift happens..
                bounds
            }
        };

        let x = bounds.x + (bounds.width / 2.0);
        let y = bounds.y + bounds.height + 10.0;

        let resize_info = get_resize_info();

        let (x, y) = resize_info.get_fixed_pos_px(x, y);

        (x - (MENU_WIDTH / 2.0), y)
    }
    pub fn get_dom_rect(&self, _resize_info: &ResizeInfo) -> web_sys::DomRect {
        //currently breaks and is also slow...
        // let bounds = match self.trace.transform.get_dom_rect_bounds() {
        //     Some(bounds) => bounds,
        //     None => {
        //         //without the real DomRects, do our best with the basic transform bounds
        //         let mut bounds = self.trace.transform.get_aabb_no_rotation_bounds_px(false);
        //         bounds.y += 10.0; //dunno why this shift happens..
        //         bounds
        //     }
        // };

        let mut bounds = self.trace.transform.get_aabb_no_rotation_bounds_px(false);
        bounds.y += 10.0; //dunno why this shift happens..

        let resize_info = get_resize_info();

        let (mut x, y) = resize_info.get_fixed_pos_px(bounds.x - (bounds.width / 2.0), bounds.y);

        if bounds.width > MENU_WIDTH {
            x += MENU_WIDTH / 2.0;
        }
        web_sys::DomRect::new_with_x_and_y_and_width_and_height(x, y, bounds.width, bounds.height)
            .unwrap_ji()
    }
}
