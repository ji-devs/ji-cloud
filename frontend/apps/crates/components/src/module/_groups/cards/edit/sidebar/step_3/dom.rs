use super::state::*;
use crate::{
    instructions::editor::dom::render as render_instructions,
    module::_groups::cards::edit::state::*,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

pub fn render<RawData, E, GetSettingsStateFn, SettingsState, RenderSettingsFn>(
    state: Rc<Step3<RawData, E, GetSettingsStateFn, SettingsState>>,
    render_settings: RenderSettingsFn,
) -> Dom
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
    RenderSettingsFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
{
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Settings),
            render_tab(state.clone(), TabKind::Instructions),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(render_settings => move |tab| {
                    match tab {
                        Tab::Settings(state) => {
                            Some(render_settings(state.clone()))
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state.clone()))
                        },
                    }
                })))
            })
        ])
    })
}

fn render_tab<RawData, E, GetSettingsStateFn, SettingsState>(
    state: Rc<Step3<RawData, E, GetSettingsStateFn, SettingsState>>,
    tab_kind: TabKind,
) -> Dom
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
{
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .property("kind", tab_kind.as_str())
        .event(clone!(state, tab_kind => move |_evt:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind, state.get_settings.clone()));
        }))
    })
}
