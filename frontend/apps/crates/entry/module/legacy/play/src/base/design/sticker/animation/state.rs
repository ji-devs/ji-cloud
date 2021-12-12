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

use awsm_web::workers::new_worker_from_js;
use dominator::clone;
use js_sys::{Object, Reflect};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
    pub last_paint_data: RefCell<Option<ImageData>>,
    pub frame_infos: RefCell<Vec<FrameInfo>>,
    pub timer: RefCell<Option<Timeout>>,
    pub raf: RefCell<Option<Raf>>
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WorkerKind {
    GifConverter,
}

static GIF_CONVERTER_SRC: &str = include_str!("gif-converter.js");

impl WorkerKind {
    pub fn make_worker(&self) -> Worker {
        match self {
            Self::GifConverter => new_worker_from_js(GIF_CONVERTER_SRC, None).unwrap_ji(),
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
    pub delay: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct GifWorkerEvent {
    pub data: GifWorkerEventData,
}

// Simple messages
// the raw image data payloads requires
// manual (de)serialization
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
enum GifWorkerEventData {
    // main -> worker: id, url
    Load(usize, String),
    // worker -> main: id, width, height, frame infos
    Init(usize, f64, f64, Vec<FrameInfo>),
}

static WORKER_ID: AtomicUsize = AtomicUsize::new(0);

impl AnimationPlayer {
    pub fn new(base: Rc<Base>, raw: RawSticker, animation: Animation) -> Rc<Self> {
        let worker_id = WORKER_ID.fetch_add(1, Ordering::SeqCst);

        let controller = Controller::new(base.clone(), &raw, animation);

        let worker = base.get_worker(WorkerKind::GifConverter);

        // log::info!("{:?}", serde_wasm_bindgen::to_value(&GifWorkerEvent{
        //     data: GifWorkerEventData::Init(10.2, 45.6)
        // }));


        let state = Rc::new(Self {
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
            last_paint_data: RefCell::new(None),
            frame_infos: RefCell::new(Vec::new()),
            frames: RefCell::new(Vec::new()),
            timer: RefCell::new(None),
            blit_time: Cell::new(0.0),
            raf: RefCell::new(None),
        });

        *state.raf.borrow_mut() = Some(Raf::new(clone!(state => move |tick| {

        })));

        *state.worker_listener.borrow_mut() = Some(EventListener::new(
            &state.worker,
            "message",
            clone!(state => move |event| {
                let event = event.dyn_ref::<web_sys::MessageEvent>().unwrap_throw();
                let data = event.data();

                if let Ok(obj_data) = Reflect::get(&data, &JsValue::from_str("data")) {
                    // ImageData payloads are slow to serde, so manually deserialize
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
                                    let _size = state.raw.override_size.unwrap_or((width, height));
                                    state.size.set(Some((width, height)));
                                }
                            },
                            _ => {}
                        }

                    }
                }
            }),
        ));

        let _ = state.worker.post_message(
            &serde_wasm_bindgen::to_value(&GifWorkerEvent {
                data: GifWorkerEventData::Load(
                    worker_id,
                    state.base.design_media_url(&state.raw.filename),
                ),
            })
            .unwrap_ji(),
        );

        state.base.insert_start_listener(clone!(state => move || {
            if state.controller.should_play_on_start {

                state.controller.curr_frame_index.store(0, Ordering::SeqCst);
                state.controller.playing.store(true, Ordering::SeqCst);
            }
        }));

        state
    }

    pub fn num_frames(&self) -> usize {
        self.frame_infos.borrow().len()
    }
    pub fn map_current_frame<A>(&self, f: impl FnOnce(usize, &FrameInfo) -> A) -> A {
        let frame_index = self.controller.curr_frame_index.load(Ordering::SeqCst);
        f(
            frame_index,
            self.frame_infos
                .borrow()
                .get(frame_index)
                .as_ref()
                .unwrap_ji(),
        )
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

    pub interactive: bool,

    pub should_play_on_start: bool
}

impl Controller {
    pub fn new(base: Rc<Base>, raw: &RawSticker, anim: Animation) -> Self {
        let interactive = raw.hide_toggle.is_some() || raw.audio_filename.is_some() || anim.tap;

        let should_play_on_start = !raw.hide && !anim.tap;
        Self {
            base,
            elem: RefCell::new(None),
            hidden: Mutable::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
            has_finished_once: AtomicBool::new(false),
            playing: AtomicBool::new(false),
            curr_frame_index: AtomicUsize::new(0),
            anim,
            hide_toggle: raw.hide_toggle,
            audio_filename: raw.audio_filename.clone(),
            interactive,
            should_play_on_start
        }
    }

}
