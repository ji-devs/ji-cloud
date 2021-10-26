use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Worker, window};
use crate::base::state::Base;
use std::io::Cursor;
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use shared::domain::jig::module::body::legacy::design::{HideToggle, Sticker as RawSticker};
use components::audio::mixer::{AUDIO_MIXER, AudioSource};

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
    // directly set from raw.hide
    pub hidden: Mutable<bool>,
    // starts false (changed via ux)
    pub has_toggled_once: AtomicBool,
    // set from raw.hide_toggle
    pub hide_toggle: Option<HideToggle>,
    pub audio_filename: Option<String>
}

impl Controller {
    pub fn new(base: Rc<Base>, raw: &RawSticker) -> Self {

        Self {
            base,
            hidden: Mutable::new(raw.hide),
            has_toggled_once: AtomicBool::new(false),
            hide_toggle: raw.hide_toggle,
            audio_filename: raw.audio_filename.clone()
        }
    }

    pub fn handle_click(&self) {
        let has_toggled_once = self.has_toggled_once.load(Ordering::SeqCst);

        if let Some(hide_toggle) = self.hide_toggle {
            if !has_toggled_once || hide_toggle == HideToggle::Always {
                let val = self.hidden.get();
                self.hidden.set(!val);
            }
        }

        self.has_toggled_once.store(true, Ordering::SeqCst);


        match (self.hidden.get(), self.audio_filename.as_ref()) {
            (false, Some(audio_filename)) => {
                AUDIO_MIXER.with(|mixer| {
                    mixer.pause_all();
                    mixer.play_oneshot(AudioSource::Url(self.base.media_url(&audio_filename)))
                });
            },
            _ => {}
        }
    }
}