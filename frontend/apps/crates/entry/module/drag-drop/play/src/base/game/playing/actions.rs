use std::rc::Rc;
use utils::math::{BoundsF64, bounds, quat, vec2};
use utils::{prelude::*, drag::Drag, resize::get_resize_info};
use super::state::*;
use shared::domain::jig::module::body::_groups::design::Sticker;
use awsm_web::{dom::StyleExt, canvas::get_2d_context};
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

            //TODO - create color lookup / map

            //First draw the target areas
            for target_area in self.game.base.target_areas.iter() {
                let trace = &target_area.trace;

                components::traces::canvas::draw_trace(&ctx, &resize_info, &trace);
                ctx.set_fill_style(&JsValue::from_str("white"));
                ctx.fill();
            }

            //Next draw our draggable's oobb
            //setting the composite rule so that it will only reveal pre-existing content
            let transform = item.curr_transform.get_cloned();
            let oobb = bounds::oobb_transform_px(true, &transform, Some(size), &resize_info);
            ctx.save();
            ctx.set_global_composite_operation(&"destination-in");
            oobb.draw_to_canvas(&ctx);
            ctx.fill();
            ctx.restore();


            //make this faster by only getting image data around the aabb
            let mut cull = oobb.to_aabb();

            //Use the bottom of the aabb instead of the top for getting the image data
            //just setting invert_y won't work (bug?) - maybe look into that or add an invert()
            //method
            let data = ctx.get_image_data(cull.left(), cull.bottom(), cull.width, cull.height) 
                .unwrap_ji()
                .data()
                .to_vec();


            let width = cull.width as usize;
            let height = cull.height as usize;

            let mut hit_color:Option<u32> = None;

            for x in 0..width {
                for y in 0..height {
                    let offset = y * (width * 4) + x * 4;
                    let r = data[offset + 0] as u32;
                    let g = data[offset + 1] as u32;
                    let b = data[offset + 2] as u32;
                    let a = data[offset + 3];

                    if a != 0 {
                        hit_color = Some(
                            (r << 16) | (g << 8) | b 
                        )
                    }
                }
            }

            if let Some(hit_color) = hit_color {
                log::info!("GOT HIT! {}", hit_color);
            }


            //Just for testing - to see the canvas
            canvas.set_style("position", "fixed");
            //canvas.set_style("opacity", "0.5");
            canvas.set_style("pointer-events", "none");
            canvas.set_style("left", &format!("{}px", resize_info.x + resize_info.content_x));
            canvas.set_style("top", &format!("{}px", resize_info.y + resize_info.content_y));

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

                self.curr_transform.replace_with(|t| {
                    let mut t = t.clone();
                    t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                    t
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
