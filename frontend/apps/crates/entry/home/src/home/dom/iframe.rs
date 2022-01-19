use dominator::{clone, html, Dom};
use dominator_helpers::events::Message;
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::HtmlIFrameElement;
use serde::{Serialize, Deserialize};

const STR_IFRAME_URL: &str = "https://www.jewishinteractive.org/jigzi-home";
const INT_IFRAME_PADDING: usize = 30;
const INT_INITIAL_HEIGHT: usize = 3000;

#[derive(Serialize, Deserialize, Debug)]
struct IframeMessageData {
    kind: String,
    height: usize,
}

pub struct Iframe {
    height: Mutable<usize>,
}

impl Iframe {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            height: Mutable::new(INT_INITIAL_HEIGHT),
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
                if let Ok(data) = event.try_serde_data::<IframeMessageData>() {
                    if data.kind == "scrollHeight" {
                        state.height.set(data.height + INT_IFRAME_PADDING);
                    }
                }
            }))
            .property("src", STR_IFRAME_URL)
        })
    }
}
