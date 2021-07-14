use std::rc::Rc;
use utils::math::{bounds, quat};
use utils::{prelude::*, drag::Drag, resize::get_resize_info};
use super::state::*;
use shared::domain::jig::module::body::_groups::design::Sticker;
use awsm_web::canvas::get_2d_context;
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

impl PlayState {
    pub fn evaluate(&self, item: &InteractiveItem) {

        if let Some(size) = item.size.get_cloned() {
            //TODO - the painting can be moved into component or utils
            //maybe canvas::paint_transform(&ctx, &transform, &size, &resize_info, &color);
            //then all we need is canvas::paint_trace the same way
            let resize_info = get_resize_info();

            let canvas:HtmlCanvasElement = web_sys::window()
                .unwrap_ji()
                .document()
                .unwrap_ji()
                .create_element("canvas")
                .unwrap_ji()
                .unchecked_into();

            canvas.set_width(resize_info.width as u32);
            canvas.set_height(resize_info.height as u32);

            let ctx = get_2d_context(&canvas, None).unwrap_ji();

            ctx.fill_rect(0.0, 0.0, resize_info.width, resize_info.height);

            // This currently works for Text
            // TODO - sprite
            let (offset_x, offset_y) = item.curr_offset.get_cloned();
            let transform = item.sticker.transform().map_offset(offset_x, offset_y);

            let bounds = bounds::transform_px(true, &transform, Some(size), &resize_info);

           
            let width = bounds.width; 
            let height = bounds.height; 

            let center_x = (bounds.x + (width/2.0));
            let center_y = (bounds.y + (height/2.0));

            //we're always assuming it's just rotated around z
            let (_, rot_rad) = quat::get_axis_angle(&transform.rotation.0);


            ctx.translate(center_x, center_y);
            ctx.rotate(rot_rad);
            ctx.translate(-center_x, -center_y);

            ctx.set_fill_style(&JsValue::from_str("white"));

            ctx.fill_rect(bounds.x, bounds.y, width, height);

            web_sys::window()
                .unwrap_ji()
                .document()
                .unwrap_ji()
                .body()
                .unwrap_ji()
                .append_child(&canvas);
        }
    }
}

impl InteractiveItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.curr_offset.replace_with(|(acc_x, acc_y)| {
                        (*acc_x - diff_x, *acc_y - diff_y)
                });
            }
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) -> bool {
        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
            //self.curr_offset.set((0.0, 0.0));
            true
        } else {
            false
        }
    }
}
