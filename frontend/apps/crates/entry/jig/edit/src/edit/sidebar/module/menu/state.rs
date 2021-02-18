use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlElement;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use crate::edit::sidebar::module::state::State as ModuleState;

pub struct State {
    pub module:Rc<ModuleState>,
    pub menu_ref:Rc<RefCell<Option<HtmlElement>>>,
}

impl State {
    pub fn new(module:Rc<ModuleState>) -> Self {
        Self {
            module,
            menu_ref: Rc::new(RefCell::new(None))
        }
    }

    pub fn close_menu(&self) {
        if let Some(menu_ref) = self.menu_ref.borrow().as_ref() {
            unsafe { Reflect::set(menu_ref, &JsValue::from_str("visible"), &JsValue::from_bool(false)); }
        }
    }

    pub fn index(&self) -> usize {
        self.module.index
    }
}
