/*
    Rough summary of what's going on:

    It takes around a full tick or more to decode a frame of GIF data
    and another tick or so to create the pixel data

    Once the pixel data exists, it can be cached and drawn quickly

    Conceptually, we handle this decoding in a worker so it doesn't tie up the UI
    but we can't actually do that since the pixel drawing part uses the Canvas API
    (offscreeen canvas isn't universally supported)

    So the first part of code complexity here is about keeping the decoding part in a worker
    while using the canvas in the main thread.
    Note that we maintain separate canvases for the work vs. painting, so it would be easy
    to move the entire processing off thread too

    We (optionally) play the GIF while it is decoding, even though it may play slowly
    until the first loop through and all the frames are cached

    In the case where the GIF begins in a paused state, we still load the frames

    The other part of the complexity is the rules regarding play vs. tap, looping or not
    This was reverse-engineered by QA and comparing to the ji tap player

    Ideally a new implementation of animation would be easier to reason about

    That said, the data structure is changed so that it's one step closer to that simplicity
*/
use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use gloo::render::AnimationFrame;
use gloo_timers::callback::Timeout;
use js_sys::{ArrayBuffer, DataView, Uint8Array};
use shared::domain::jig::module::body::legacy::design::{Animation, HideToggle, Sticker as RawSticker};
use std::borrow::Borrow;
use std::convert::TryInto;
use std::ops::{Mul, Sub};
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlImageElement, Element, ImageData, Worker, window};
use crate::base::state::Base;
use std::io::Cursor;
use std::cell::{Cell, Ref};
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use js_sys::{Object, Reflect};
use components::audio::mixer::{AUDIO_MIXER, AudioSource};
use awsm_web::{
    loaders::fetch::fetch_url,
    workers::new_worker_from_js,
};

pub struct AnimationPlayer {
    pub base: Rc<Base>,
    pub raw: RawSticker,
    pub size: Mutable<Option<(f64, f64)>>,
    pub controller: Controller,
    pub worker_id: usize,
    pub worker: Worker,
    pub worker_listener: RefCell<Option<EventListener>>,
    pub paint_ctx: RefCell<Option<CanvasRenderingContext2d>>,
    pub work_ctx: RefCell<Option<CanvasRenderingContext2d>>,
    pub work_canvas: RefCell<Option<HtmlCanvasElement>>,
    pub blit_time: Cell<f64>,
    pub frames: RefCell<Vec<ImageData>>,
    pub prev_frame_info: RefCell<Option<FrameInfo>>,
    pub prev_frame_data: RefCell<Option<ImageData>>,
    pub frame_infos: RefCell<Vec<FrameInfo>>,
    pub timer: RefCell<Option<Timeout>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WorkerKind {
    GifConverter,
}

static GIF_CONVERTER_SRC:&str = include_str!("gif-converter.js");

impl WorkerKind {
    pub fn make_worker(&self) -> Worker {
        match self {
            Self::GifConverter => {
                new_worker_from_js(GIF_CONVERTER_SRC, None).unwrap_ji()
            },
        }
    } 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameInfo {
    pub data_length: usize, 
    pub data_offset: usize,
    pub disposal: u8, 
    pub has_local_palette: bool, 
    pub height: u32,
    pub interlaced: bool, 
    pub palette_offset: usize,
    pub palette_size: usize,
    pub transparent_index: usize,
    pub width: u32,
    pub x: u32,
    pub y: u32,
    pub delay: f64 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct GifWorkerEvent {
    pub data: GifWorkerEventData
}

// Simple messages
// the raw image data payloads requires
// manual (de)serialization 
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "data", rename_all="snake_case")]
enum GifWorkerEventData {
    // main -> worker: id, url
    Load(usize, String),
    // worker -> main: id, width, height, frame infos 
    Init(usize, f64, f64, Vec<FrameInfo>),
}

static WORKER_ID:AtomicUsize = AtomicUsize::new(0);

impl AnimationPlayer {
    pub fn new(base: Rc<Base>, raw: RawSticker, animation: Animation) -> Rc<Self> {



        let worker_id = WORKER_ID.fetch_add(1, Ordering::SeqCst);

        let controller = Controller::new(base.clone(), &raw, animation);

        let worker = base.get_worker(WorkerKind::GifConverter);

            // log::info!("{:?}", serde_wasm_bindgen::to_value(&GifWorkerEvent{
            //     data: GifWorkerEventData::Init(10.2, 45.6)
            // }));

        let state = Rc::new(Self{
            base,
            raw,
            //wait until we have frame_info, even if using the override
            size: Mutable::new(None),
            controller,
            worker,
            worker_id,
            worker_listener: RefCell::new(None),
            paint_ctx: RefCell::new(None),
            work_ctx: RefCell::new(None),
            work_canvas: RefCell::new(None),
            prev_frame_info: RefCell::new(None), 
            prev_frame_data: RefCell::new(None), 
            frame_infos: RefCell::new(Vec::new()),
            frames: RefCell::new(Vec::new()),
            timer: RefCell::new(None),
            blit_time: Cell::new(0.0),
        });

        *state.worker_listener.borrow_mut() = Some(EventListener::new(&state.worker, "message", clone!(state => move |event| {
            let event = event.dyn_ref::<web_sys::MessageEvent>().unwrap_throw();
            let data = event.data();

            if let Ok(obj_data) = Reflect::get(&data, &JsValue::from_str("data")) {
                /// ImageData payloads are slow to serde, so manually deserialize
                if Reflect::get(&obj_data, &JsValue::from_str("kind")) == Ok(JsValue::from_str("frame_resp")) {
                    let data = Reflect::get(&obj_data, &JsValue::from_str("data")).unwrap_ji();
                    let id = Reflect::get(&data, &JsValue::from_str("id")).unwrap_ji().as_f64().unwrap_ji() as usize;
                    let img_data:ImageData = Reflect::get(&data, &JsValue::from_str("img_data")).unwrap_ji().unchecked_into();
                    if id == state.worker_id {
                        state.clone().paint(&img_data, true);
                        state.frames.borrow_mut().push(img_data);
                    }
                } else if let Ok(msg) = serde_wasm_bindgen::from_value::<GifWorkerEvent>(data) {
                    match msg.data {
                        GifWorkerEventData::Init(id, width, height, frame_infos)=> {
                            if id == state.worker_id {
                                *state.frame_infos.borrow_mut() = frame_infos; 

                                //this will cause the canvas to be created
                                //which in turn will make the first frame request
                                let size = state.raw.override_size.unwrap_or((width, height));
                                state.size.set(Some((width, height)));
                            }
                        },
                        _ => {}
                    }

                }
            }
        })));

        state.worker.post_message(&serde_wasm_bindgen::to_value(&GifWorkerEvent{
            data: GifWorkerEventData::Load(worker_id, state.base.design_media_url(&state.raw.filename))
        }).unwrap_ji());

        state
    }

    pub fn num_frames(&self) -> usize {
        self.frame_infos.borrow().len()
    }
    pub fn map_current_frame<A>(&self, f: impl FnOnce(usize, &FrameInfo) -> A) -> A {
        let frame_index = self.controller.curr_frame_index.load(Ordering::SeqCst);
        f(frame_index, self.frame_infos.borrow().get(frame_index).as_ref().unwrap_ji())
    }

    pub fn request_frame(self: Rc<Self>) {
        self.blit_time.set(Self::curr_time());


        self.map_current_frame(|frame_index, frame_info| {
            if let Some(img_data) = self.frames.borrow().get(frame_index) {
                self.clone().paint(img_data, false);
            } else {
                let img_data = self.prep_cache_frame();

                // manually constructing due to binary buffer
                let obj = Object::new();
                let data = Object::new();
                let payload = Object::new();

                Reflect::set(&payload, &JsValue::from_str("img_data"), &img_data);
                Reflect::set(&payload, &JsValue::from_str("frame_index"), &JsValue::from_f64(frame_index as f64));
                Reflect::set(&payload, &JsValue::from_str("id"), &JsValue::from_f64(self.worker_id as f64));
                Reflect::set(&data, &JsValue::from_str("kind"), &JsValue::from_str("frame_req"));
                Reflect::set(&data, &JsValue::from_str("data"), &payload);
                Reflect::set(&obj, &JsValue::from_str("data"), &data);
                
                self.worker.post_message(&obj).unwrap_ji();
            }

        });

    }

    fn curr_time() -> f64 {
       web_sys::window().unwrap_ji().performance().unwrap_ji().now()
    }

    // based on: https://github.com/movableink/omggif/blob/example-web/example_web/index.html
    fn prep_cache_frame(&self) -> ImageData {
        //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
        let ctx = self.work_ctx.borrow();
        let ctx = ctx.as_ref().unwrap_ji();

        let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

        self.clear_cache_frame();
        // this is a fairly expensive operation, takes like 5-15ms
        let buffer = ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height).unwrap_ji();

        //log::info!("prep time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        buffer
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
                    },
                    1 => {
                        // "Do not dispose" - do nothing, we draw over the existing canvas
                    },
                    2 => {
                        // "Restore to background" - browsers ignore background color, so
                        // in practice it is always "Restore to transparent"
                        ctx.clear_rect(
                            prev_frame_info.x as f64, 
                            prev_frame_info.y as f64,
                            prev_frame_info.width as f64,
                            prev_frame_info.height as f64
                        );
                    },
                    3 => {
                        // "Restore to previous" - revert back to most recent frame that was
                        // not set to "Restore to previous", or frame 0
                        log::info!("should restore!");
                        if let Some(prev_frame_data) = self.prev_frame_data.borrow().as_ref() {
                            ctx.put_image_data(prev_frame_data, 0.0, 0.0);
                        }
                    },
                    _ => {}
                }
            }

            if frame_index == 0 || prev_frame_info.map(|x| x.disposal < 2) == Some(true) {
                *self.prev_frame_data.borrow_mut() = Some(
                    // this is a fairly expensive operation, takes like 5-15ms
                    ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height).unwrap_ji()
                );
            }


            //log::info!("clear time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });
    }

    fn cached_all_frames(&self) -> bool {
        self.frames.borrow().len() >= self.frame_infos.borrow().len()
    }
    fn paint(self: Rc<Self>, img_data: &ImageData, write_cache: bool) {

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
                    &img_data,
                    0.0,
                    0.0,
                    frame_info.x as f64, 
                    frame_info.y as f64,
                    frame_info.width as f64,
                    frame_info.height as f64
                ).unwrap_ji();
            }

            let ctx = self.paint_ctx.borrow();
            let ctx = ctx.as_ref().unwrap_ji();

            if (frame_index == 0 && write_cache) || self.controller.playing.load(Ordering::SeqCst) {
                ctx.put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(
                    &img_data,
                    0.0,
                    0.0,
                    frame_info.x as f64, 
                    frame_info.y as f64,
                    frame_info.width as f64,
                    frame_info.height as f64
                ).unwrap_ji();
            }
            //log::info!("blit time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });

        self.next_frame();
    }

    fn next_frame(self: Rc<Self>) {
        let frame_index = self.controller.curr_frame_index.fetch_add(1, Ordering::SeqCst);

        if frame_index == self.num_frames() - 1 {
            if self.controller.playing.load(Ordering::SeqCst) {
                self.controller.has_finished_once.store(true, Ordering::SeqCst);

                if self.controller.anim.once {
                    self.controller.playing.store(false, Ordering::SeqCst);
                }
            }

            self.controller.curr_frame_index.store(0, Ordering::SeqCst);
        }

        let state = self;
        let delay = 
            state.frame_infos.borrow().get(frame_index).unwrap_ji().delay
                .mul(10.0)
                .sub(Self::curr_time() - state.blit_time.get())
                .max(0.0);


        // log::info!("{}", delay);

        //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();

        *state.timer.borrow_mut() = Some(Timeout::new(delay as u32, clone!(state => move || {
            state.request_frame();
        })));
    }

}

pub struct Controller {
    pub base: Rc<Base>,
    pub elem: RefCell<Option<Element>>,
    // store the settings
    pub anim: Animation,
    // directly set from raw.hide
    pub hidden: Mutable<bool>,
    // starts false (changed via ux)
    pub has_toggled_once: AtomicBool,
    // starts false (changed via ux)
    pub has_finished_once: AtomicBool,
    // starts 0 (changed via internal updates)
    pub curr_frame_index: AtomicUsize,
    // starts as _not_ anim.tap... e.g. start
    // playing right away if not waiting for a tap
    // changes under these conditions:
    // 1. set to false when animation ended and _once_ is true (i.e. no loop)
    // 2. toggled on tap 

    pub playing: AtomicBool,
    // set from raw.hide_toggle
    pub hide_toggle: Option<HideToggle>,

    pub audio_filename: Option<String>,

    pub interactive: bool
}

impl Controller {
    pub fn new(base: Rc<Base>, raw: &RawSticker, anim: Animation) -> Self {

        let interactive = raw.hide_toggle.is_some() || raw.audio_filename.is_some() || anim.tap;

        Self {
            base,
            elem: RefCell::new(None),
            hidden: Mutable::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
            has_finished_once: AtomicBool::new(false),
            playing: AtomicBool::new(!raw.hide && !anim.tap),
            curr_frame_index: AtomicUsize::new(0),
            anim,
            hide_toggle: raw.hide_toggle,
            audio_filename: raw.audio_filename.clone(),
            interactive
        }
    }
}