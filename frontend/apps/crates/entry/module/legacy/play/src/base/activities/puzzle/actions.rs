use super::state::*;
use dominator::clone;
use std::rc::Rc;
use utils::{prelude::*, drag::Drag, resize::{ResizeInfo, get_resize_info}, math::{mat_2d, mat4::{self, Matrix4}, vec2}};
use components::traces::{canvas::{draw_single_shape, apply_transform_mat4, clip_single_shape}, utils::TraceShapeExt};
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use crate::config::PUZZLE_DISTANCE_THRESHHOLD;

impl Puzzle {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip(state.base.activity_media_url(&audio_filename));
        }

        state.base.allow_stage_click();
    }

    pub fn create_pieces(&self) {

    }

    pub fn render_cutouts(&self) {

    }
}

impl PuzzleGame {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();
    }

    pub fn draw(&self, resize_info: &ResizeInfo) {
        let canvas = &self.cutouts_canvas;
        let ctx = &self.cutouts_ctx;

        canvas.set_width(resize_info.width as u32);
        canvas.set_height(resize_info.height as u32);

        //draw complete background
        ctx.draw_image_with_html_image_element_and_dw_and_dh(&self.effects.image_element, 0.0, 0.0, resize_info.width, resize_info.height).unwrap_ji();

        //draw the cutouts
        ctx.set_fill_style(&JsValue::from_str("black"));
        for item in self.items.iter() {
            draw_single_shape(ctx, resize_info, &item.raw.hotspot.shape, );
        }

        //draw the items
        for item in self.items.iter() {
            ctx.save();

            let mut mat = item.curr_transform_matrix.borrow().clone();
            mat.denormalize(&resize_info);
            apply_transform_mat4(&ctx, &mat);

            clip_single_shape(&ctx, resize_info, &item.raw.hotspot.shape);

            ctx.draw_image_with_html_image_element_and_dw_and_dh(&self.effects.image_element, 0.0, 0.0, resize_info.width, resize_info.height).unwrap_ji();

            ctx.restore();
        }

        if self.drag_index.get().is_none() {
            self.draw_click_detection(resize_info)
        }
    }

    //this is unfortunately expensive, not sure why though.
    //in any case, should only run when waiting for a click
    pub fn draw_click_detection(&self, resize_info: &ResizeInfo) {

        let canvas = &self.click_canvas;
        let ctx = &self.click_ctx;

        canvas.set_width(resize_info.width as u32);
        canvas.set_height(resize_info.height as u32);

        ctx.clear_rect(0.0, 0.0, resize_info.width, resize_info.height);


        for (index, item) in self.items.iter().enumerate() {


            let r = 0xFF & (index >> 16);
            let g = 0xFF & (index >> 8);
            let b = 0xFF & index;

            // let color = {
            //     if index == 0 {
            //         "red".to_string()
            //     } else if index == 1 {
            //         "green".to_string()
            //     } else {
            //         "yellow".to_string()
            //     }
            // };
            
            let color = format!("#{:02x}{:02x}{:02x}", r, g, b);

            ctx.save();

            let mut mat = item.curr_transform_matrix.borrow().clone();
            mat.denormalize(&resize_info);
            apply_transform_mat4(&ctx, &mat);

            ctx.set_fill_style(&JsValue::from_str(&color));
            if !draw_single_shape(&ctx, resize_info, &item.raw.hotspot.shape) {
                ctx.fill();
            }
            ctx.restore();
        }
    }


    pub fn start_drag(&self, x: i32, y: i32) {

        let resize_info = get_resize_info();
        let canvas_x = (x as f64) - resize_info.x;
        let canvas_y = (y as f64) - resize_info.y;

        if let Ok(data) = self.click_ctx.get_image_data(canvas_x, canvas_y, 1.0, 1.0) {
            let data = data.data().to_vec();

            let r = data[0] as u32;
            let g = data[1] as u32;
            let b = data[2] as u32;
            let a = data[3];

            //we use alpha just to check if there _is_ a hit
            if a != 0 {
                let index = ((r << 16) | (g << 8) | b) as usize;
                let item = &self.items[index];
                if !item.completed.get() {
                    self.items[index].start_drag(x, y);
                    self.drag_index.set(Some(index));
                }
                //log::info!("got hit! {}, mouse: {} {}, canvas: {} {}", index, x, y, canvas_x, canvas_y);
            } else {
                //log::info!("not hit! mouse: {} {}, canvas: {} {}", x, y, canvas_x, canvas_y);
            }
        }
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(index) = self.drag_index.get() {
            self.items[index].try_move_drag(x, y);
            self.draw(&get_resize_info());
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) {
        if let Some(index) = self.drag_index.get() {
            let item = &self.items[index];

            if item.try_end_drag(x, y) {
                item.evaluate(self.raw.fly_back_to_origin);
            }

            //could potentially animate, which is why we preserve
            //drag_index for now, otherwise it'll draw the clickable
            //areas
            self.draw(&get_resize_info());

            self.drag_index.set(None);
            self.draw(&get_resize_info());

            self.evaluate_all();
        }
    }

    pub fn evaluate_all(&self) {
        if self.items.iter().all(|item| item.completed.get()) {
            log::info!("all finished!!");
            let msg = match self.raw.jump_index {
                Some(index) => {
                    log::info!("going to index {}!", index);
                    IframeAction::new(ModuleToJigPlayerMessage::JumpToIndex(index))
                }
                None => {
                    log::info!("going next!");
                    IframeAction::new(ModuleToJigPlayerMessage::Next)
                }
            };

            let _ = msg.try_post_message_to_top();
        }
    }
}

impl PuzzleItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        if !self.completed.get() {
            *self.drag.borrow_mut() = Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true)));
        }
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.borrow_mut().as_mut() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.curr_transform_matrix.replace_with(|m| {
                    m.add_translation(&[diff_x * -1.0, diff_y * -1.0, 0.0]);
                    m.clone()
                });
            }
        }
    }

    pub fn try_end_drag(&self, _x: i32, _y: i32) -> bool {
        if let Some(_drag) = self.drag.borrow_mut().take() {
            true
        } else {
            false
        }
    }

    pub fn evaluate(&self, fly_back_to_origin: bool) {
        let curr_t = self.curr_transform_matrix.borrow().get_translation();
        let dist = vec2::distance(&curr_t, &[0.0, 0.0]);

        if(dist <= PUZZLE_DISTANCE_THRESHHOLD) {
            *self.curr_transform_matrix.borrow_mut() = Matrix4::identity(); 

            self.completed.set(true);
        } else {
            if fly_back_to_origin {
                *self.curr_transform_matrix.borrow_mut() = self.orig_transform_matrix.clone();
            }
        }
    }
}

