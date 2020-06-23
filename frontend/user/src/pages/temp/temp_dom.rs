use std::rc::Rc;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use dominator::{Dom, svg, class, text, html, clone, events, link};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal}
};
use crate::path;

pub struct TempDom { }
impl TempDom {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
        })
    }
    pub fn render(comp: Rc<Self>) -> Dom {
        html!("div", {
            .text("TODO!")
        })
    }
}
