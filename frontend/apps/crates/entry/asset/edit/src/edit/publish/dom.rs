use super::{post_publish::PostPublish, pre_publish::PrePublish, Publish};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

impl Publish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .prop("slot", "main")
            .child_signal(state.post_publish.signal().map(clone!(state => move |post_publish| {
                Some(if !post_publish {
                    PrePublish::render(Rc::clone(&state))
                } else {
                    PostPublish::new(Rc::clone(&state.asset_edit_state)).render()
                })
            }))
            )
        })
    }
}
