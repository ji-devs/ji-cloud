use std::rc::Rc;
use dominator::Dom;
use crate::module::{
    edit::prelude::*,
    _groups::cards::edit::state::*,
};
use super::{
    step_1::state::Step1,
    step_2::state::Step2,
    step_3::state::Step3,
};

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
    pub render_settings: RenderSettingsStateFn
}

impl <RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> Sidebar <RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> 
where
    RawData: RawDataExt, 
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
    pub fn new(base: Rc<CardsBase<RawData, E>>, get_settings: GetSettingsStateFn, render_settings: RenderSettingsStateFn) -> Self {
        Self {
            base,
            get_settings,
            render_settings
        }
    }
}

impl <RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> SidebarExt for Sidebar<RawData, E, GetSettingsStateFn, RenderSettingsStateFn, SettingsState> 
where
    RawData: RawDataExt, 
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    RenderSettingsStateFn: Fn(Rc<SettingsState>) -> Dom + Clone + 'static,
    SettingsState: 'static,
{
}
