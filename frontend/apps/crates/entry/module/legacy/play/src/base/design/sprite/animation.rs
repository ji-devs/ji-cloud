use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use gloo_timers::callback::Timeout;
use js_sys::{ArrayBuffer, DataView, Uint8Array};
use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite,
    Animation
};
use std::convert::TryInto;
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Worker, window};
use crate::base::state::{Base, WorkerKind};
use std::io::Cursor;
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

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
    pub num_frames: AtomicUsize,
    pub curr_frame_index: AtomicUsize,
    pub frame_infos: RefCell<Vec<FrameInfo>>,
    pub timer: RefCell<Option<Timeout>>,
}

#[derive(Serialize, Deserialize, Debug)]
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

// Serialized messages
// the raw data buffer skips this system and requires
// manual deserialization to strip the id
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "data", rename_all="snake_case")]
enum GifWorkerEventData {
    // main -> worker: id, url
    Load(usize, String),
    // worker -> main: id, width, height, number of frames, frame infos 
    Init(usize, f64, f64, usize, Vec<FrameInfo>),
    // main -> worker: id, frame index 
    FrameReq(usize, usize),
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
            num_frames: AtomicUsize::new(0),
            curr_frame_index: AtomicUsize::new(0),
            frame_infos: RefCell::new(Vec::new()),
            timer: RefCell::new(None),
        });

        *state.worker_listener.borrow_mut() = Some(EventListener::new(&state.worker, "message", clone!(state => move |event| {
            let event = event.dyn_ref::<web_sys::MessageEvent>().unwrap_throw();
            let data = event.data();

            /// sending a large buffer via serialization is slow
            /// so we handle this as a special case
            if ArrayBuffer::is_view(&data) {
                //get it as a buffer
                let buffer = data.unchecked_into::<Uint8Array>();
                //extract the id
                let view = DataView::new(&buffer.buffer(), (buffer.length() - 4).try_into().unwrap_ji(), 4);
                let id = view.get_uint32(0) as usize;

                if id == state.worker_id {
                    // get the pixels-only buffer
                    let pixels = buffer.subarray(0, buffer.length() - 4);

                    let frame_index = state.curr_frame_index.load(Ordering::SeqCst);
                    //this is concerning! shouldn't happen...
                    if let Some(frame_info) = state.frame_infos.borrow().get(frame_index) {
                        state.paint(frame_info, pixels);
                        state.clone().next_frame();
                    }
                }
            } else if let Ok(msg) = serde_wasm_bindgen::from_value::<GifWorkerEvent>(data) {
                match msg.data {
                    GifWorkerEventData::Init(id, width, height, num_frames, frame_infos)=> {
                        if id == state.worker_id {
                            //log::info!("width: {}, height: {}, num_frames: {}", width, height, num_frames);
                            state.num_frames.store(num_frames, Ordering::SeqCst);
                            state.size.set(Some((width, height)));

                            *state.frame_infos.borrow_mut() = frame_infos; 

                            state.clone().request_frame();
                        }
                    },
                    _ => {}
                }

            }
        })));

        state.worker.post_message(&serde_wasm_bindgen::to_value(&GifWorkerEvent{
            data: GifWorkerEventData::Load(worker_id, state.base.media_url(format!("{}.gif", state.raw.src)))
        }).unwrap_ji());

        state
    }

    pub fn next_frame(self: Rc<Self>) {
        let frame_index = self.curr_frame_index.fetch_add(1, Ordering::SeqCst);
        if frame_index == self.num_frames.load(Ordering::SeqCst) - 1 {
            self.curr_frame_index.store(0, Ordering::SeqCst);
        }

        let state = self;
        let mut delay = state.frame_infos.borrow().get(frame_index).unwrap_ji().delay;

        *state.timer.borrow_mut() = Some(Timeout::new(delay as u32, clone!(state => move || {
            state.request_frame();
        })));
    }
    pub fn request_frame(self: Rc<Self>) {
        let frame_index = self.curr_frame_index.load(Ordering::SeqCst);

        self.worker.post_message(&serde_wasm_bindgen::to_value(&GifWorkerEvent{
            data: GifWorkerEventData::FrameReq(self.worker_id, frame_index)
        }).unwrap_ji());

        let state = self;
    }

    fn paint(&self, frame_info: &FrameInfo, pixels: Uint8Array) {

        if let Some(ctx) = self.ctx.borrow().as_ref() {
            //let start = web_sys::window().unwrap_ji().performance().unwrap_ji().now();
            let (canvas_width, canvas_height) = self.size.get_cloned().unwrap_ji();

            // log::info!("{}", pixels.length());

            //TODO - disposal?
            //ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);


            let img_data = ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&pixels.to_vec()), // would be nice to just use the typed array without a copy...
                //pixels,
                frame_info.width,
                frame_info.height,
            )
            .unwrap_ji();

            ctx.put_image_data(&img_data, frame_info.x as f64, frame_info.y as f64);
            //log::info!("draw time: {}", web_sys::window().unwrap_ji().performance().unwrap_ji().now() - start);
        }

            
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