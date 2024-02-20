use dominator::{clone, html, Dom};
use dominator_helpers::events::Message;
use futures_signals::signal::{Mutable, SignalExt};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use utils::prelude::get_user_id;
use web_sys::HtmlIFrameElement;

const STR_IFRAME_URL: &str = "https://www.jewishinteractive.org/jigzi-home";
const STR_IFRAME_LOGGED_IN_URL: &str = "https://www.jewishinteractive.org/jigzi-home-logged-in";
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
        let state = self;

        let iframe_src = match get_user_id() {
            Some(_) => STR_IFRAME_LOGGED_IN_URL,
            None => STR_IFRAME_URL,
        };

        html!("iframe" => HtmlIFrameElement, {
            .style("width", "100%")
            .style("border", "none")
            .style_signal("height", state.height.signal()
                .map(|height| {

                    height.to_string() + "px"
                })
            )
            .global_event(clone!(state => move |event: Message| {
                if let Ok(data) = event.try_serde_data::<IframeMessageData>() {
                    if data.kind == "scrollHeight" {
                        state.height.set(data.height + INT_IFRAME_PADDING);
                    }
                }
            }))
            .prop("src", iframe_src)
        })
    }
}
