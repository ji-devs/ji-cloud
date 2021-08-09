use crate::traces::edit::draw::trace::state::*;
use utils::resize::ResizeInfo;

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
                let mut bounds = self.trace.transform.get_aabb_bounds_px(false);
                bounds.y += 10.0; //dunno why this shift happens..
                bounds
            }
        };

        let x = bounds.x + (bounds.width / 2.0);
        let y = bounds.y + bounds.height + 10.0;

        (x - (MENU_WIDTH / 2.0), y)
    }
}
