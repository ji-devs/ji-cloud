use super::{post_publish::PostPublish, pre_publish::PrePublish, Publish};
use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use std::rc::Rc;

impl Publish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .property("slot", "main")
            .child_signal(
                map_ref! {
                    let asset = state.asset.signal_cloned(),
                    let post_publish = state.post_publish.signal() => move {
                        (asset.clone(), *post_publish)
                    }
                }
                .map(clone!(state => move |(asset, post_publish)| {
                    asset.map(clone!(state => move |asset| {
                        if !post_publish {
                            PrePublish::render(Rc::clone(&state), asset)
                        } else {
                            PostPublish::new(asset, Rc::clone(&state.asset_edit_state)).render()
                        }
                    }))
                }))
            )
        })
    }
}
