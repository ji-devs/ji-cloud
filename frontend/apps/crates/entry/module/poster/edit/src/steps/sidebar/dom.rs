use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("div", {
            .text_signal(state.step_state.signal_cloned().map(|step_state| {
                if let Some(step_state) = step_state.as_ref() {
                    match step_state.as_ref() {
                        StepState::One => "step 1 here!",
                        StepState::Two => "step 2 here!",
                        StepState::Three => "step 3 here!",
                    }
                } else {
                    "nothing!"
                }
            }))
        })
    }
}
