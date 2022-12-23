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
    has_feedback: bool,
) -> Dom
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
    RenderSettingsFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
{
    state.base.can_continue_next.set_neq(true);
    state
        .base
        .continue_next_fn
        .set(Some(Rc::new(clone!(state => move || {
            if let Some(kind) = state.next_kind() {
                state.tab.set(Tab::new(state.base.clone(), kind, state.get_settings.clone()));
                true
            } else {
                false
            }
        }))));

    let mut tabs = vec![
        render_tab(state.clone(), MenuTabKind::PlaySettings),
        render_tab(state.clone(), MenuTabKind::Instructions),
    ];

    if has_feedback {
        tabs.push(render_tab(state.clone(), MenuTabKind::Feedback));
    }

    tabs.push(html!("module-sidebar-body", {
        .prop("slot", "body")
        .style("overflow", "inherit") // Inherit overflow otherwise the Hebrew controls will be hidden
        .child_signal(state.tab.signal_cloned().map(clone!(render_settings => move |tab| {
            match tab {
                Tab::Settings(state) => {
                    Some(render_settings(state))
                },
                Tab::Instructions(state) => {
                    Some(render_instructions(state))
                },
                Tab::Feedback(state) => {
                    Some(render_instructions(state))
                }
            }
        })))
    }));

    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.kind()).dedupe().for_each(clone!(state => move |kind| {
            state.tab_kind.set(Some(kind));
            async move {}
        })))
        .children(tabs)
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
            true,
            true,
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
