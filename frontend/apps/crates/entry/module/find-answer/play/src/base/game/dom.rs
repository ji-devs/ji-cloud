use super::{
    playing::{dom::render as render_playing, state::*},
    state::*,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render(state: Rc<Game>) -> Dom {
    html!("empty-fragment", {
        .child_signal(state.question.signal_cloned().map(clone!(state => move |question| {
            if let Some((_index, question)) = question {
                Some(render_playing(PlayState::new(state.clone(), question)))
            } else {
                None
            }
        })))
        // TODO Remove this
        // .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
        //     match phase {
        //         // Phase::ShowHints => Some(render_hints(Hints::new(state.clone()))),
        //         Phase::Playing => Some(render_playing(PlayState::new(state.clone()))),
        //     }
        // })))
    })
}
