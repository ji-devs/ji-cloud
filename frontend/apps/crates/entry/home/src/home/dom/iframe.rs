use dominator::{clone, events, html, with_node, Dom};
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
                    log::info!("Height: {}", adjusted_height);
                    adjusted_height
                })
            )
            .with_node!(elem => {
                .event(clone!(state => move |_: events::Load| {
                    match get_height(&elem) {
                        Ok(height) => state.height.set(height),
                        Err(_) => (),
                    }

                }))
            })
            .property("src", STR_JEWISH_INTERACTIVE_URL)
        })
    }
}

#[wasm_bindgen]
pub fn get_height(iframe: &HtmlIFrameElement) -> Result<usize, JsValue> {
    let height: usize = match iframe.content_window() {
        Some(content_window) => {
            match content_window.post_message(&JsValue::from_str("scrollHeight"), STR_TARGET_DOMAIN) {
                Ok(_) => log::info!("post_message worked"),
                Err(error) => log::info!("post_message failed\n{:?}", error),
            };
            3000
        },
        None => 3000
    };
    
    Ok(height)

    // iframe.post_message("", "");
}
