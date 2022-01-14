use dominator::{events, html, clone, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

const STR_JEWISH_INTERACTIVE_URL: &str = "https://www.jewishinteractive.org/jigzi-home";

pub struct Iframe {
    height: Mutable<usize>,
}

impl Iframe {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            height: Mutable::new(400),
        })
    }
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self.clone();
        html!("iframe", {
            .style("width", "100%")
            .style_signal("height", state.height.signal_cloned()
                .map(|height| {
                    let adjusted_height = height.to_string() + "px";
                    log::info!("Height: {}", adjusted_height);
                    adjusted_height
                })
            )
            .event(clone!(state => move |_: events::Load| {
                log::info!("loaded");
                state.height.set(1200);
            }))
            .property("src", STR_JEWISH_INTERACTIVE_URL)
        })
    }
}
