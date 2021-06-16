use components::module::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers, traces};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use super::pair::{
    state::MainPair,
    dom::render as render_pair,
};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child_signal(state.base.is_empty_signal().map(clone!(state => move |is_empty| {
                Some(
                    if is_empty {
                        html!("main-empty")
                    } else {
                        html!("main-cards", {
                            .children_signal_vec({
                                state.base.step.signal()
                                    .switch_signal_vec(clone!(state => move |step| {
                                        state.base.pairs
                                            .signal_vec_cloned()
                                            .enumerate()
                                            .map(clone!(state => move |(index, pair)| {
                                                let pair = MainPair::new(
                                                    state.clone(),
                                                    step.clone(),
                                                    index.clone(),
                                                    pair
                                                );
                                                render_pair(pair)
                                            }))
                                    }))
                            })
                        })
                    }
                )
            })))
        })
    }
}
