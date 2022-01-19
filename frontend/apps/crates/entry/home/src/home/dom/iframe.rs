use dominator::{clone, html, Dom};
use dominator_helpers::events::Message;
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::HtmlIFrameElement;

const STR_JEWISH_INTERACTIVE_URL: &str = "https://www.jewishinteractive.org/jigzi-home";

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
            .style_signal("height", state.height.signal()
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
