use super::state::*;
use utils::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use components::traces::bubble::state::TraceBubble;
use dominator::clone;

impl Step3 {

    pub fn start_preview(&self, index: usize) {

        let bounds = self
            .base
            .traces
            .get(index)
            .unwrap_ji()
            .bounds
            .get()
            .unwrap_ji()
            .clone();

        let trace_meta = self
            .base
            .traces_meta
            .lock_ref()
            .get(index)
            .unwrap_ji()
            .clone();

        let audio = trace_meta.audio.get_cloned();
        let text = trace_meta.text.get_cloned();

        TraceBubble::set_unset_mutable(bounds, audio, text, trace_meta.bubble.clone());
    }
}
