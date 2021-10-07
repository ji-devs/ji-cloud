use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use dominator::Dom;
use std::rc::Rc;
use futures_signals::{map_ref, signal::{Signal, SignalExt, Mutable}};

pub struct Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    pub base: Rc<CardsBase<RawData, E>>,
    pub get_settings: GetSettingsStateFn,
    pub render_settings: RenderSettingsStateFn,
}

impl<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
    Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    pub fn new(
        base: Rc<CardsBase<RawData, E>>,
        get_settings: GetSettingsStateFn,
        render_settings: RenderSettingsStateFn,
    ) -> Self {
        Self {
            base,
            get_settings,
            render_settings,
        }
    }
}

impl<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> SidebarExt
    for Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    type TabIndexSignal = impl Signal<Item = Option<usize>>;

    fn tab_index(&self) -> Self::TabIndexSignal {
        futures_signals::signal::always(None)
    }

}
