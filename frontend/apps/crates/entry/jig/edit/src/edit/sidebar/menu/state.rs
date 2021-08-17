use futures_signals::signal::Mutable;
use js_sys::Reflect;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct State {
    pub menu_ref: Rc<RefCell<Option<HtmlElement>>>,
    pub dup_as_active: Mutable<bool>,
}

impl State {
    pub fn new() -> Self {
        Self {
            menu_ref: Rc::new(RefCell::new(None)),
            dup_as_active: Mutable::new(false),
        }
    }

    pub fn close_menu(&self) {
        if let Some(menu_ref) = self.menu_ref.borrow().as_ref() {
            let _ = Reflect::set(
                menu_ref,
                &JsValue::from_str("visible"),
                &JsValue::from_bool(false),
            );
        }
        self.dup_as_active.set(false);
    }
}
