use dominator::{events, html, Dom};
use futures_signals::signal::Mutable;
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
        html!("iframe", {
            .style("width", "100%")
            .event(|_: events::Load| {
                log::info!("loaded");
            })
            .property("src", STR_JEWISH_INTERACTIVE_URL)
        })
    }
}
