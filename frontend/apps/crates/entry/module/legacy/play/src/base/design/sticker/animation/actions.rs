use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use awsm_web::tick::Raf;
use gloo_timers::callback::Timeout;

use shared::domain::jig::module::body::legacy::design::{
    Animation, HideToggle, Sticker as RawSticker,
};

use crate::base::state::Base;
use std::ops::{Mul, Sub};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, ImageData, Worker};

use dominator::clone;
use js_sys::{Object, Reflect};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::state::*;
use crate::base::actions::StageClick;
use utils::math::bounds::BoundsF64;

impl Controller {
    pub fn handle_click(&self, stage_click: StageClick) {
        let is_target = {
            match self.elem.borrow().as_ref() {
                None => false,
                Some(elem) => {
                    let bounds: BoundsF64 = elem.into();
                    bounds.contains_point(stage_click.mouse_x, stage_click.mouse_y)
                }
            }
        };

        if !is_target || !self.interactive {
            return;
        }

        let has_toggled_once = self.has_toggled_once.load(Ordering::SeqCst);

        if let Some(hide_toggle) = self.hide_toggle {
            if !has_toggled_once || hide_toggle == HideToggle::Always {
                let val = self.hidden.get();
                self.hidden.set(!val);
            }
        }

        self.has_toggled_once.store(true, Ordering::SeqCst);

        let (playing_anim, playing_audio) = if self.hidden.get() {
            (false, false)
        } else {
            let play_toggle = !self.playing.load(Ordering::SeqCst);

            if self.anim.tap {
                (play_toggle, play_toggle)
            } else if self.anim.once && self.has_finished_once.load(Ordering::SeqCst) {
                (false, play_toggle)
            } else {
                (true, true)
            }
        };

        self.playing.store(playing_anim, Ordering::SeqCst);

        if playing_anim {
            // this is a small departure from TT, reset to the beginning in case
            // the sound was a bit timed to the animation
            if self.anim.tap {
                self.curr_frame_index.store(0, Ordering::SeqCst);
            }
        }

        if playing_audio {
            if let Some(audio_filename) = self.audio_filename.as_ref() {
                //win the race condition with hotspots
                self.base
                    .audio_manager
                    .play_clip_next_tick(self.base.design_media_url(&audio_filename));
            }
        }
    }
}


impl AnimationPlayer {

    pub fn request_frame(self: Rc<Self>) {
        self.blit_time.set(Self::curr_time());

        self.map_current_frame(|frame_index, _frame_info| {
            if let Some(img_data) = self.frames.borrow().get(frame_index) {
                self.clone().paint(img_data, false);
            } else {
                let img_data = self.prep_cache_frame();

                // manually constructing due to binary buffer
                let obj = Object::new();
                let data = Object::new();
                let payload = Object::new();

                let _ = Reflect::set(&payload, &JsValue::from_str("img_data"), &img_data);
                let _ = Reflect::set(
                    &payload,
                    &JsValue::from_str("frame_index"),
                    &JsValue::from_f64(frame_index as f64),
                );
                let _ = Reflect::set(
                    &payload,
                    &JsValue::from_str("id"),
                    &JsValue::from_f64(self.worker_id as f64),
                );
                let _ = Reflect::set(
                    &data,
                    &JsValue::from_str("kind"),
                    &JsValue::from_str("frame_req"),
                );
                let _ = Reflect::set(&data, &JsValue::from_str("data"), &payload);
                let _ = Reflect::set(&obj, &JsValue::from_str("data"), &data);

                self.worker.post_message(&obj).unwrap_ji();
            }
        });
    }
    pub fn repaint_for_hidden(&self, hidden: bool) {

        let ctx = self.paint_ctx.borrow();
        let ctx = ctx.as_ref().unwrap_ji();

        if hidden {
            ctx.clear_rect(0.0, 0.0, ctx.canvas().unwrap().width().into(), ctx.canvas().unwrap().height().into());
        } else {
            if let Some(img_data) = self.last_paint_data.borrow().as_ref() {
                self.map_current_frame(|frame_index, frame_info| {
                    ctx.put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
                        img_data,
                        0.0,
                        0.0,
                        frame_info.x as f64,
                        frame_info.y as f64,
                        frame_info.width as f64,
                        frame_info.height as f64,
                    )
                    .unwrap_ji();
                })
            }
        }
    }
    pub fn paint(self: Rc<Self>, img_data: &ImageData, write_cache: bool) {
        self.map_current_frame(|frame_index, frame_info| {
            if write_cache {
                //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
                let ctx = self.work_ctx.borrow();
                let ctx = ctx.as_ref().unwrap_ji();

                let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();
                if frame_index == 0 {
                    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
                }

                //ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
                ctx.put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
                    img_data,
                    0.0,
                    0.0,
                    frame_info.x as f64,
                    frame_info.y as f64,
                    frame_info.width as f64,
                    frame_info.height as f64,
                )
                .unwrap_ji();
            }

            let ctx = self.paint_ctx.borrow();
            let ctx = ctx.as_ref().unwrap_ji();

            if !self.controller.hidden.get() && ((frame_index == 0 && write_cache) || self.controller.playing.load(Ordering::SeqCst))  {
                ctx.put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
                    img_data,
                    0.0,
                    0.0,
                    frame_info.x as f64,
                    frame_info.y as f64,
                    frame_info.width as f64,
                    frame_info.height as f64,
                )
                .unwrap_ji();
            }
            //log::info!("blit time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });

        *self.last_paint_data.borrow_mut() = Some(img_data.clone());

        self.next_frame();
    }


    fn curr_time() -> f64 {
        web_sys::window()
            .unwrap_ji()
            .performance()
            .unwrap_ji()
            .now()
    }

    // based on: https://github.com/movableink/omggif/blob/example-web/example_web/index.html
    fn prep_cache_frame(&self) -> ImageData {
        //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
        let ctx = self.work_ctx.borrow();
        let ctx = ctx.as_ref().unwrap_ji();

        let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

        self.clear_cache_frame();
        // this is a fairly expensive operation, takes like 5-15ms

        //log::info!("prep time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height)
            .unwrap_ji()
    }

    fn clear_cache_frame(&self) {
        self.map_current_frame(|frame_index, frame_info| {
            //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
            let ctx = self.work_ctx.borrow();
            let ctx = ctx.as_ref().unwrap_ji();

            let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

            if frame_index == 0 {
                ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
            }

            let prev_frame_info = self.prev_frame_info.replace(Some(frame_info.clone()));

            if let Some(prev_frame_info) = prev_frame_info.as_ref() {
                match prev_frame_info.disposal {
                    0 => {
                        // "No disposal specified" - do nothing, we draw over the existing canvas
                    }
                    1 => {
                        // "Do not dispose" - do nothing, we draw over the existing canvas
                    }
                    2 => {
                        // "Restore to background" - browsers ignore background color, so
                        // in practice it is always "Restore to transparent"
                        ctx.clear_rect(
                            prev_frame_info.x as f64,
                            prev_frame_info.y as f64,
                            prev_frame_info.width as f64,
                            prev_frame_info.height as f64,
                        );
                    }
                    3 => {
                        // "Restore to previous" - revert back to most recent frame that was
                        // not set to "Restore to previous", or frame 0
                        log::info!("should restore!");
                        if let Some(prev_frame_data) = self.prev_frame_data.borrow().as_ref() {
                            let _ = ctx.put_image_data(prev_frame_data, 0.0, 0.0);
                        }
                    }
                    _ => {}
                }
            }

            if frame_index == 0 || prev_frame_info.map(|x| x.disposal < 2) == Some(true) {
                *self.prev_frame_data.borrow_mut() = Some(
                    // this is a fairly expensive operation, takes like 5-15ms
                    ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height)
                        .unwrap_ji(),
                );
            }

            //log::info!("clear time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });
    }

    fn _cached_all_frames(&self) -> bool {
        self.frames.borrow().len() >= self.frame_infos.borrow().len()
    }

    fn next_frame(self: Rc<Self>) {
        let frame_index = self
            .controller
            .curr_frame_index
            .fetch_add(1, Ordering::SeqCst);

        if frame_index == self.num_frames() - 1 {
            if self.controller.playing.load(Ordering::SeqCst) {
                self.controller
                    .has_finished_once
                    .store(true, Ordering::SeqCst);

                if self.controller.anim.once {
                    self.controller.playing.store(false, Ordering::SeqCst);
                }
            }

            self.controller.curr_frame_index.store(0, Ordering::SeqCst);
        }

        let state = self;
        let delay = state
            .frame_infos
            .borrow()
            .get(frame_index)
            .unwrap_ji()
            .delay
            .mul(10.0)
            .sub(Self::curr_time() - state.blit_time.get())
            .max(0.0);

        // log::info!("{}", delay);

        //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();

        *state.timer.borrow_mut() = Some(Timeout::new(
            delay as u32,
            clone!(state => move || {
                state.request_frame();
            }),
        ));
    }
}
