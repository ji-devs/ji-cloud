use super::pair::{dom::render as render_pair, state::MainPair};
use super::state::*;
use crate::{
    backgrounds::dom::render_single_background,
    module::{_common::edit::prelude::*, _groups::cards::edit::state::*},
};
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::jig::module::body::_groups::cards::Step;
use std::rc::Rc;

impl<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> DomRenderable
    for Main<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    fn render(
        state: Rc<Main<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>>,
    ) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.base.is_empty_signal().map(clone!(state => move |is_empty| {
                Some(
                    if is_empty {
                        html!("main-empty")
                    } else {
                        html!("empty-fragment", {
                            .child_signal(state.base.step.signal_cloned().map(clone!(state => move |step| {
                                // Reset card selection when changing step.
                                state.base.selected_pair.set(None);

                                Some(match step {
                                    Step::Three => {
                                        (state.render_settings) (Rc::new((state.get_settings) (state.base.clone())))
                                    },
                                    _ => {
                                        render_main_cards(state.base.clone(), step)
                                    }
                                })
                            })))
                        })
                    }
                )
            })))
        })
    }
}
impl<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> MainDomRenderable
    for Main<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    fn render_bg(
        state: Rc<Main<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>>,
    ) -> Option<Dom> {
        Some(render_single_background(
            state.base.background.signal_cloned(),
            state.base.theme_id.signal_cloned(),
            None,
        ))
    }
}

pub fn render_main_cards<RawData: RawDataExt, E: ExtraExt>(
    base: Rc<CardsBase<RawData, E>>,
    step: Step,
) -> Dom {
    html!("main-cards", {
        .children_signal_vec({
            base.pairs
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(base => move |(index, pair)| {
                    let pair = MainPair::new(
                        base.clone(),
                        step,
                        index,
                        pair
                    );
                    render_pair(pair)
                }))
        })
    })
}
