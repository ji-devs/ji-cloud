use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use gloo_timers::callback::Timeout;
use js_sys::{ArrayBuffer, DataView, Uint8Array};
use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite,
    Animation
};
use std::borrow::Borrow;
use std::convert::TryInto;
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Worker, window};
use crate::base::state::{Base, WorkerKind};
use std::io::Cursor;
use std::cell::{Cell, Ref};
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use js_sys::{Object, Reflect};

pub struct AnimationPlayer {
    pub base: Rc<Base>,
    pub raw: RawSprite,
    pub size: Mutable<Option<(f64, f64)>>,
    pub hide: HideController,
    pub anim: AnimationController,
    pub worker_id: usize,
    pub worker: Worker,
    pub worker_listener: RefCell<Option<EventListener>>,
    pub ctx: RefCell<Option<CanvasRenderingContext2d>>,
    pub curr_frame_index: AtomicUsize,
    pub prev_frame_info: RefCell<Option<FrameInfo>>,
    pub prev_frame_data: RefCell<Option<ImageData>>,
    pub frame_infos: RefCell<Vec<FrameInfo>>,
    pub timer: RefCell<Option<Timeout>>,
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
    pub fn new(base: Rc<Base>, raw: RawSprite, animation: Animation) -> Rc<Self> {

        let worker_id = WORKER_ID.fetch_add(1, Ordering::SeqCst);

        let hide = HideController::new(&raw);
        let anim = AnimationController::new(&raw, animation);

        let worker = base.get_worker(WorkerKind::GifConverter);

            // log::info!("{:?}", serde_wasm_bindgen::to_value(&GifWorkerEvent{
            //     data: GifWorkerEventData::Init(10.2, 45.6)
            // }));

        let state = Rc::new(Self{
            base,
            raw,
            size: Mutable::new(None),
            hide,
            anim,
            worker,
            worker_id,
            worker_listener: RefCell::new(None),
            ctx: RefCell::new(None),
            curr_frame_index: AtomicUsize::new(0),
            prev_frame_info: RefCell::new(None), 
            prev_frame_data: RefCell::new(None), 
            frame_infos: RefCell::new(Vec::new()),
            timer: RefCell::new(None),
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
                        state.blit(img_data);
                        state.clone().next_frame();
                    }
                } else if let Ok(msg) = serde_wasm_bindgen::from_value::<GifWorkerEvent>(data) {
                    match msg.data {
                        GifWorkerEventData::Init(id, width, height, frame_infos)=> {
                            if id == state.worker_id {
                                *state.frame_infos.borrow_mut() = frame_infos; 

                                //this will cause the canvas to be created
                                //which in turn will make the first frame request
                                state.size.set(Some((width, height)));

                            }
                        },
                        _ => {}
                    }

                }
            }
        })));

        state.worker.post_message(&serde_wasm_bindgen::to_value(&GifWorkerEvent{
            data: GifWorkerEventData::Load(worker_id, state.base.media_url(format!("{}.gif", state.raw.src)))
        }).unwrap_ji());

        state
    }

    pub fn num_frames(&self) -> usize {
        self.frame_infos.borrow().len()
    }
    pub fn map_current_frame<A>(&self, f: impl FnOnce(usize, &FrameInfo) -> A) -> A {
        let frame_index = self.curr_frame_index.load(Ordering::SeqCst);
        f(frame_index, self.frame_infos.borrow().get(frame_index).as_ref().unwrap_ji())
    }

    pub fn next_frame(self: Rc<Self>) {
        let frame_index = self.curr_frame_index.fetch_add(1, Ordering::SeqCst);
        if frame_index == self.num_frames() - 1 {
            self.curr_frame_index.store(0, Ordering::SeqCst);
        }

        let state = self;
        let mut delay = state.frame_infos.borrow().get(frame_index).unwrap_ji().delay;

        *state.timer.borrow_mut() = Some(Timeout::new(delay as u32, clone!(state => move || {
            state.request_frame();
        })));
    }
    pub fn request_frame(self: Rc<Self>) {

        self.map_current_frame(|frame_index, frame_info| {
            let img_data = self.prep_frame();

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

        });

    }

    // based on: https://github.com/movableink/omggif/blob/example-web/example_web/index.html
    fn prep_frame(&self) -> ImageData {
        let ctx = self.ctx.borrow();
        let ctx = ctx.as_ref().unwrap_ji();

        let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

        let stash = ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height).unwrap_ji();
        self.clear_frame();
        // this is a fairly expensive operation, takes like 5-15ms
        let buffer = ctx.get_image_data(0.0, 0.0, canvas_width, canvas_height).unwrap_ji();

        ctx.put_image_data(&stash, 0.0, 0.0);

        buffer
    }

    fn clear_frame(&self) {
        self.map_current_frame(|frame_index, frame_info| {
            //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
            let ctx = self.ctx.borrow();
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


            //log::info!("blit time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });
    }
    fn blit(&self, img_data: ImageData) {
        self.map_current_frame(|frame_index, frame_info| {
            //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
            let ctx = self.ctx.borrow();
            let ctx = ctx.as_ref().unwrap_ji();

            let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

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

            //log::info!("blit time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        });
    }
}

pub struct AnimationController {
    pub playing: Mutable<bool>,
    pub settings: Animation,
}

impl AnimationController {
    pub fn new(raw: &RawSprite, settings: Animation) -> Self {
        Self {
            playing: Mutable::new(!settings.tap),
            settings
        }
    }
}
pub struct HideController {
    pub is_hidden: AtomicBool,
    pub has_toggled_once: AtomicBool,
}

impl HideController {
    pub fn new(raw: &RawSprite) -> Self {
        Self {
            is_hidden: AtomicBool::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
        }
    }
}