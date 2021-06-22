use dominator::{clone, html, Dom};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;
use super::{
    step_1::{
        dom::render as render_step_1,
        state::Step1
    },
    step_2::{
        dom::render as render_step_2,
        state::Step2
    },
    step_3::{
        dom::render as render_step_3,
        state::Step3
    },
};
use crate::module::{
    _groups::cards::edit::state::*,
    edit::prelude::*,
};
use shared::domain::jig::module::body::_groups::cards::Step;

impl <RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> DomRenderable for Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> 
where
    RawData: RawDataExt, 
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    fn render(state: Rc<Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>>) -> Dom {
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
