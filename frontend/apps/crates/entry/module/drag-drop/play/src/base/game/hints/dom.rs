use super::state::*;
use std::rc::Rc;
use super::state::*;
use components::traces;
use gloo_timers::future::TimeoutFuture;
use dominator::{clone, html, Dom};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
pub fn render(state: Rc<Hints>) -> Dom {

    html!("empty-fragment", {
        .future(clone!(state => async move {
            TimeoutFuture::new(crate::config::HINT_TIME).await;
            state.finish();
        }))
        .child(traces::hints::dom::render(
                state.game.base.traces
                    .iter()
                    .map(|t| t.trace.clone())
                    .collect()
        ))
    })
}
