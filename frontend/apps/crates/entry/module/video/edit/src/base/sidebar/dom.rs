use super::state::*;
use components::module::_common::edit::prelude::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::video::Step;
use std::rc::Rc;

use super::{
    step_1::{dom::render as render_step_1, state::Step1},
    step_2::{dom::render as render_step_2, state::Step2},
    step_3::{dom::render as render_step_3, state::Step3},
};

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("empty-fragment", {
            .style("display", "contents")
            .child_signal(state.base.step.signal_cloned().map(clone!(state => move |step| {
                match step {
                    Step::One => Some(render_step_1(Step1::new(state.base.clone()))),
                    Step::Two => Some(render_step_2(Step2::new(state.base.clone()))),
                    Step::Three => Some(render_step_3(Step3::new(state.base.clone()))),
                    _ => None
                }
            })))
        })
    }
}
