use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Element, ImageData, Worker, window};
use crate::base::state::Base;
use std::io::Cursor;
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use shared::domain::jig::module::body::legacy::design::{HideToggle, Sticker as RawSticker};

pub struct ImagePlayer {
    pub base: Rc<Base>,
    pub raw: RawSticker,
    pub size: Mutable<Option<(f64, f64)>>,
    pub controller: Controller,
}

impl ImagePlayer {
    pub fn new(base: Rc<Base>, raw: RawSticker) -> Rc<Self> {

        let size = Mutable::new(raw.override_size);
        let controller = Controller::new(base.clone(), &raw);

        Rc::new(Self{
            base,
            raw,
            controller,
            size
        })
    }

}

pub struct Controller {
    pub base: Rc<Base>,
    pub elem: RefCell<Option<Element>>,
    // directly set from raw.hide
    pub hidden: Mutable<bool>,
    // starts false (changed via ux)
    pub has_toggled_once: AtomicBool,
    // set from raw.hide_toggle
    pub hide_toggle: Option<HideToggle>,
    pub audio_filename: Option<String>,
    pub interactive: bool,
}

impl Controller {
    pub fn new(base: Rc<Base>, raw: &RawSticker) -> Self {

        let interactive = raw.hide_toggle.is_some() || raw.audio_filename.is_some();

        Self {
            base,
            elem: RefCell::new(None),
            hidden: Mutable::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
            hide_toggle: raw.hide_toggle,
            audio_filename: raw.audio_filename.clone(),
            interactive
        }
    }
}