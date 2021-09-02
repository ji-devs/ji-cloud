use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use super::state::*;
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use super::pages::{
    start::dom::StartPage,
    step_1::dom::Step1Page,
    step_2::dom::Step2Page,
    step_3::dom::Step3Page,
};

pub struct RegisterPage {
}

impl RegisterPage {
    pub fn render(step: Option<Step>, is_no_auth: bool) -> Dom {
        let state = Rc::new(State::new(step));

        html!("empty-fragment", {
            .child_signal(state.step.signal_cloned().map(clone!(state, is_no_auth => move |step| {
                match step {
                    Step::Start => Some(StartPage::render(state.step.clone(), is_no_auth)),
                    Step::One(data) => Some(Step1Page::render(state.step.clone(), data)),
                    Step::Two(data) => Some(Step2Page::render(state.step.clone(), data)),
                    Step::Three(data) => Some(Step3Page::render(state.step.clone(), data)),
                }
            })))
        })
    }
}

