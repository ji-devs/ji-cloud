use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("div", {
            .child_signal(state.step_state.signal_cloned().map(|step_state| {
                step_state
                    .map(|step_state| {
                        match step_state {
                            StepState::One(one) => super::step_1::dom::render(one),
                            StepState::Two(two) => super::step_2::dom::render(two),
                            StepState::Three(three) => super::step_3::dom::render(three),
                            StepState::Four(four) => super::step_4::dom::render(four),
                        }
                    })
            }))
        })
    }
}
