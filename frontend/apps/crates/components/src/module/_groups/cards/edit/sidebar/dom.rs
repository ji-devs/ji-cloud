use super::state::*;
use super::{
    step_1::{dom::render as render_step_1, state::Step1},
    step_2::{dom::render as render_step_2, state::Step2},
    step_3::{dom::render as render_step_3, state::Step3},
};
use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::_groups::cards::Step;
use std::rc::Rc;

impl<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> DomRenderable
    for Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    fn render(
        state: Rc<Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>>,
    ) -> Dom {
        html!("div", {
            .child_signal(state.base.step.signal_cloned().map(clone!(state => move |step| {
                match step {
                    Step::One => Some(render_step_1(Step1::new(state.base.clone()))),
                    Step::Two => Some(render_step_2(Step2::new(state.base.clone()))),
                    Step::Three => Some(
                        render_step_3(
                            Step3::new(state.base.clone(), state.get_settings.clone()),
                            state.render_settings.clone()
                        )
                    ),
                    _ => None
                }
            })))
        })
    }
}
