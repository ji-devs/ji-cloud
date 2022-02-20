/*
    Click detection is pixel-perfect even with advanced shapes
    The way we do it is, instead of creating a polygon around it,
    we render each shape to an offscreen canvas, where the color
    is derived directly from the shape's index.

    For example, shape 0's R value is 0, shape 1's R value is 1
    and this goes on, filling RGB sequentially

    Then, when the screen is clicked, we read the pixel at that position
    and reverse the process to get the shape index

    It _might_ be possible to use alpha, but it's unclear whether this 
    skews the internal data with blending, canvas composite mode, etc.

    So we only support up to 16,777,216 shapes... should be more than enough ;)

    See pixels in stickers_traces or Drag and Drop player for a similar technique,
    but there it is being used for shape-to-shape collision detection
*/

use super::state::*;
use dominator::{clone, animation::Percentage};
use std::rc::Rc;
use futures_signals::{
    signal::{SignalExt}
};
use utils::{prelude::*, drag::Drag, resize::{ResizeInfo, get_resize_info}, math::{mat4::{Matrix4}, vec2}};
use components::traces::{canvas::{draw_single_shape, apply_transform_mat4, clip_single_shape}};
use crate::base::actions::NavigationTarget;
use wasm_bindgen::prelude::*;
use crate::config::PUZZLE_DISTANCE_THRESHHOLD;

impl Puzzle {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        if state.raw.show_preview {
            log::info!("showing preview...");
        }
        if let Some(audio_filename) = state.raw.audio_filename.as_ref() {
            state.base.audio_manager.play_clip(state.base.activity_media_url(&audio_filename));
        }

        state.base.allow_stage_click();
    }

}

impl PuzzleGame {

    pub fn with_all_items_ref(&self, f: impl Fn(&PuzzleItem)) {
        let locked_items = self.locked_items.borrow();
        let free_items = self.free_items.borrow();

        for item in locked_items.iter() {
            f(item);
        }


        if let Some(active_index) = self.drag_index.get() {
            for item in free_items
                .iter()
                .enumerate()
                .filter(|(idx, item)| *idx != active_index)
                .map(|(_, item)| item) {
                    f(item);
                }

            f(&free_items[active_index]);
        } else {
            for item in free_items.iter() {
                f(item);
            }
        }

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
        for item in self.free_items.borrow().iter() {
            draw_single_shape(ctx, resize_info, &item.raw.hotspot.shape, );
        }

        //draw the items
        self.with_all_items_ref(|item| {
            ctx.save();

            let mut mat = item.curr_transform_matrix.borrow().clone();
            mat.denormalize(&resize_info);
            apply_transform_mat4(&ctx, &mat);

            clip_single_shape(&ctx, resize_info, &item.raw.hotspot.shape);

            ctx.draw_image_with_html_image_element_and_dw_and_dh(&self.effects.image_element, 0.0, 0.0, resize_info.width, resize_info.height).unwrap_ji();

            ctx.restore();
        });
    }

    //this is unfortunately expensive, not sure why though.
    //in any case, should only run when waiting for a click
    pub fn draw_click_detection(&self, resize_info: &ResizeInfo) {

        let canvas = &self.click_canvas;
        let ctx = &self.click_ctx;

        canvas.set_width(resize_info.width as u32);
        canvas.set_height(resize_info.height as u32);

        ctx.clear_rect(0.0, 0.0, resize_info.width, resize_info.height);


        for (index, item) in self.free_items.borrow().iter().enumerate() {


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
        self.draw_click_detection(&resize_info);

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
                self.free_items.borrow()[index].start_drag(x, y);
                self.drag_index.set(Some(index));
                //log::info!("got hit! {}, mouse: {} {}, canvas: {} {}", index, x, y, canvas_x, canvas_y);
            } else {
                //log::info!("not hit! mouse: {} {}, canvas: {} {}", x, y, canvas_x, canvas_y);
            }
        }
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(index) = self.drag_index.get() {
            self.free_items.borrow()[index].try_move_drag(x, y);
            self.draw(&get_resize_info());
        }
    }

    pub fn try_end_drag(&self, x: i32, y: i32) {
        if let Some(index) = self.drag_index.get() {
            let item = self.free_items.borrow()[index].clone();

            if item.try_end_drag(x, y) {
                if item.evaluate(self.raw.fly_back_to_origin) {
                    self.free_items.borrow_mut().remove(index);
                    self.locked_items.borrow_mut().push(item);
                }
            }

            self.drag_index.set(None);
            self.draw(&get_resize_info());

            self.evaluate_all();
        }
    }

    pub fn evaluate_all(&self) {
        if self.free_items.borrow().len() == 0 {
            log::info!("all finished!!");
            match self.raw.jump_index {
                Some(index) => {
                    log::info!("going to index {}!", index);
                    self.base.navigate(NavigationTarget::Index(index));
                }
                None => {
                    log::info!("going next!");
                    self.base.navigate(NavigationTarget::Next);
                }
            };

        }
    }
}

impl PuzzleItem {
    pub fn start_drag(&self, x: i32, y: i32) {
        *self.drag.borrow_mut() = Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true)));

        if let Some(audio_filename) = self.raw.audio_filename.as_ref() {
            self.base.audio_manager.play_clip(self.base.activity_media_url(audio_filename));
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

    pub fn evaluate(&self, fly_back_to_origin: bool) -> bool {
        let curr_t = self.curr_transform_matrix.borrow().get_translation();
        let dist = vec2::distance(&curr_t, &[0.0, 0.0]);

        if dist <= PUZZLE_DISTANCE_THRESHHOLD {
            *self.curr_transform_matrix.borrow_mut() = Matrix4::identity(); 

            self.base.audio_manager.play_positive_clip();
            true
        } else {
            if fly_back_to_origin {
                *self.curr_transform_matrix.borrow_mut() = self.orig_transform_matrix.clone();
            }
            self.base.audio_manager.play_negative_clip();
            false
        }
    }
}


impl PuzzlePreview {
    pub fn start_animation(self: Rc<Self>, parent: Rc<Puzzle>) {
        let state = self;

        state.loader.load(
            state.animation.signal().for_each(clone!(state, parent => move |t| {

                state.draw_animation(t);

                if t == Percentage::END {
                    parent.init_phase.set(InitPhase::Playing(state.game.clone()));
                }

                async {}
            }))
        );

        state.animation.animate_to(Percentage::END);
    }

    pub fn draw_animation(&self, perc:Percentage) {

        let t = perc.into_f64();
        self.game.with_all_items_ref(|item| {
            let mut v = item.orig_transform_matrix.get_translation();
            v[0] *= t;
            v[1] *= t;

            let m = &mut *item.curr_transform_matrix.borrow_mut();
            m.translate(&v);
        });

        self.game.draw(&get_resize_info());
    }
}
