use super::state::*;
use utils::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use components::traces::bubble::state::TraceBubble;
use dominator::clone;

impl Step3 {

    //Will update if either kind or selected index changed 
    pub fn activate_tab(&self, kind: TabKind) {

        if let Some(index) = self.base.traces.selected_index.get_cloned() {
            let mut go_ahead = false;

            let curr_index = self.tab_trace_index.borrow().clone();
            if Some(index) != curr_index {
                go_ahead = true;
            }
            if let Some(curr_kind) = self.tab.lock_ref().as_ref().map(|tab| tab.kind()) {
                if curr_kind != kind {
                    go_ahead = true;
                }
            }

            if go_ahead {
                *self.tab_trace_index.borrow_mut() = Some(index);
                self.tab.set(Some(Tab::new(self.base.clone(), kind, index)));
            }

        }

    }

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
        let bubble = trace_meta.bubble.clone();

        let instance = Rc::new(TraceBubble::new(
            bounds, 
            audio, 
            text,
            Some(clone!(bubble => move || {
                bubble.set(None)
            }))
        ));

        bubble.set(Some(instance));
    }
}
