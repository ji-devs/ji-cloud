use std::rc::Rc;
use dominator::{clone, html, Dom};
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use super::{
    state::*,
    hints::{
        dom::render as render_hints,
        state::*
    },
    playing::{
        dom::render as render_playing,
        state::*
    },
};
pub fn render(state: Rc<Game>) -> Dom {
    html!("empty-fragment", {
        .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
            match phase {
                Phase::ShowHints => Some(render_hints(Hints::new(state.clone()))),
                Phase::Playing => Some(render_playing(PlayState::new(state.clone()))),
            }
        })))
    })
}



