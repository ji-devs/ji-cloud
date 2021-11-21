use super::state::*;
use dominator::clone;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;

impl TalkType {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();
    }
}


impl TalkTypeItem {
    pub fn evaluate(self: Rc<Self>, parent: Rc<TalkType>) {
        let state = self;
    }
}