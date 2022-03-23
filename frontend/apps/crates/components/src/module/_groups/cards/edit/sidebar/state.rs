use crate::{module::{_common::edit::prelude::*, _groups::cards::edit::state::*}, tabs::MenuTabKind};
use dominator::Dom;
use futures_signals::signal::{Mutable, Signal};
use std::rc::Rc;

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
    pub tab_kind: Mutable<Option<MenuTabKind>>,
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
            tab_kind: Mutable::new(None),
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
    type TabKindSignal = impl Signal<Item = Option<MenuTabKind>>;

    fn tab_kind(&self) -> Self::TabKindSignal {
        self.tab_kind.signal()
    }
}
