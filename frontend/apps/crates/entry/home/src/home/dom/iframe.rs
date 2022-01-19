use dominator::{clone, events, html, with_node, Dom};
use dominator_helpers::events::Message;
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlIFrameElement;
use js_sys::Function;

const STR_JEWISH_INTERACTIVE_URL: &str = "https://www.jewishinteractive.org/jigzi-home";
const STR_TARGET_DOMAIN: &str = "https://www.jewishinteractive.org/";

pub struct Iframe {
    height: Mutable<usize>,
}

impl Iframe {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            height: Mutable::new(3000),
        })
    }
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self.clone();
        html!("iframe" => HtmlIFrameElement, {
            .style("width", "100%")
            .style("border", "none")
            .style_signal("height", state.height.signal_cloned()
                .map(|height| {
                    let adjusted_height = height.to_string() + "px";
                    adjusted_height
                })
            )
            .global_event(clone!(state => move |event: Message| {
                if let Ok(height) = event.try_serde_data::<String>() {
                    state.height.set(height.parse::<usize>().unwrap());
                }
            }))
            .property("src", STR_JEWISH_INTERACTIVE_URL)
        })
    }
}
