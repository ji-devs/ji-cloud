use super::state::*;
use components::module::_common::edit::prelude::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::drag_drop::Step;
use std::rc::Rc;

use super::{
    step_1::{dom::render_step_1, state::Step1},
    step_2::{dom::render_step_2, state::Step2},
    step_3::dom::render_step_3,
    step_4::dom::render_step_4,
    step_5::{dom::render_step_5, state::Step5},
};

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("empty-fragment", {
            .future(state.base.step.signal_cloned().dedupe().for_each(clone!(state => move |_step| {
                state.tab_kind.set(None);
                async move {}
            })))
            .style("display", "contents")
            .child_signal(state.base.step.signal_cloned().map(clone!(state => move |step| {
                match step {
                    Step::One => Some(render_step_1(Step1::new(state.clone()))),
                    Step::Two => Some(render_step_2(Step2::new(state.clone()))),
                    Step::Three => Some(render_step_3()),
                    Step::Four => Some(render_step_4()),
                    Step::Five => Some(render_step_5(Step5::new(state.clone()))),
                    _ => None
                }
            })))
        })
    }
}
