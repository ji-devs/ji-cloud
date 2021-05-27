use utils::{prelude::*, resize::ResizeInfo, math::BoundsF64};
use crate::traces::edit::draw::trace::state::*;

#[derive(Clone)]
pub struct Menu {
    pub trace: DrawTrace 
}

//in normalized units
const MENU_MARGIN: f64 = 0.01;

//in real-world units
const MENU_WIDTH: f64 = 217.0;

impl Menu {
    pub fn new(trace: DrawTrace) -> Self {
        Self { trace }
    }

    pub fn get_pos(&self, resize_info: &ResizeInfo) -> (f64, f64) {

        let bounds = match self.trace.transform.get_dom_rect_bounds() {
            Some(bounds) => bounds,
            None => {
                //without the real DomRects, do our best with the basic transform bounds
                //this will actually work fine in every case here
                //since the initial transform is always non-rotated
                self.trace.transform.get_bounds_px(false)
            }
        };

        let x = bounds.x + (bounds.width/2.0);
        let y = bounds.y + bounds.height + 10.0; 

        (
            x - (MENU_WIDTH / 2.0),
            y
        )
    }
}
