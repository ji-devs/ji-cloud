use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::routes::RegisterQuery;
use std::rc::Rc;

use super::pages::{
    start::RegisterStart, step_1::dom::Step1Page, step_2::dom::Step2Page, step_3::dom::Step3Page,
};

pub struct RegisterPage {}

impl RegisterPage {
    pub fn render(step: Option<Step>, query: RegisterQuery) -> Dom {
        let state = Rc::new(State::new(step));

        html!("empty-fragment", {
            .child_signal(state.step.signal_cloned().map(clone!(state => move |step| {
                match step {
                    Step::Start => Some(RegisterStart::new(state.step.clone(), query.clone()).render()),
                    Step::One(data) => Some(Step1Page::render(state.step.clone(), data)),
                    Step::Two(data) => Some(Step2Page::render(state.step.clone(), data)),
                    Step::Three(data) => Some(Step3Page::render(state.step.clone(), data)),
                }
            })))
        })
    }
}
