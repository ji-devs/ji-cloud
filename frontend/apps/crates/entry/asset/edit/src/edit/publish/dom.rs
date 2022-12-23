use super::{post_publish::PostPublish, pre_publish::PrePublish, Publish};
use dominator::{clone, html, Dom};
use std::rc::Rc;

impl Publish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .prop("slot", "main")
            .child_signal(state.published_asset.signal_ref(clone!(state => move |published_asset| {
                Some(match published_asset {
                    None => {
                        PrePublish::render(Rc::clone(&state))
                    },
                    Some(asset) => {
                        PostPublish::new(Rc::clone(&state.asset_edit_state), asset).render()
                    },
                })
            })))
        })
    }
}
