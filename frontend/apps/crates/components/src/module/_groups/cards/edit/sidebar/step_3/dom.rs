use super::state::*;
use crate::{
    instructions::editor::dom::render as render_instructions,
    module::_groups::cards::edit::state::*,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

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
        .future(state.tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
            state.tab_index.set(Some(index));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::PlaySettings),
            render_tab(state.clone(), MenuTabKind::Instructions),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(render_settings => move |tab| {
                    match tab {
                        Tab::Settings(state) => {
                            Some(render_settings(state))
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state))
                        },
                    }
                })))
            })
        ])
    })
}

fn render_tab<RawData, E, GetSettingsStateFn, SettingsState>(
    state: Rc<Step3<RawData, E, GetSettingsStateFn, SettingsState>>,
    tab_kind: MenuTabKind,
) -> Dom
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
{
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            false,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.base.clone(), tab_kind, state.get_settings.clone()));
            }),
        ),
        Some("tabs"),
    )
}
