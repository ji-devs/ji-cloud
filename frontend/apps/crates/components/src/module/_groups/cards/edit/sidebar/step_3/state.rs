use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use crate::{
    instructions::editor::{
        state::State as InstructionsEditorState,
        callbacks::Callbacks as InstructionsEditorCallbacks
    },
    module::_groups::cards::edit::{
        state::*,
        config,
        strings
    }
};

pub struct Step3<RawData, E, GetSettingsStateFn, SettingsState> 
where
    RawData: RawDataExt, 
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
{
    pub base: Rc<CardsBase<RawData, E>>,
    pub tab: Mutable<Tab<SettingsState>>,
    pub get_settings: GetSettingsStateFn,
}


impl <RawData, E, GetSettingsStateFn, SettingsState> Step3<RawData, E, GetSettingsStateFn, SettingsState> 
where
    RawData: RawDataExt, 
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
{
    pub fn new(base: Rc<CardsBase<RawData, E>>, get_settings: GetSettingsStateFn) -> Rc<Self> {
        let kind = match base.debug.step3_tab {
            Some(kind) => kind,
            None => TabKind::Settings
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind, get_settings.clone()));

        Rc::new(Self {
            base,
            tab,
            get_settings
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Settings,
    Instructions,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Settings => "play-settings",
            Self::Instructions => "instructions",
        }
    }
}

pub enum Tab<SettingsState> {
    Settings(Rc<SettingsState>),
    Instructions(Rc<InstructionsEditorState>),
}

//the generic makes the auto derive break
impl <SettingsState> Clone for Tab<SettingsState> {
    fn clone(&self) -> Self {
        match self {
            Self::Settings(state) => Self::Settings(state.clone()),
            Self::Instructions(state) => Self::Instructions(state.clone()),
        }
    }
}

impl <SettingsState> Tab <SettingsState> {
    pub fn new<RawData, E, GetSettingsStateFn>(base: Rc<CardsBase<RawData, E>>, kind:TabKind, get_settings: GetSettingsStateFn) -> Self 
    where
        RawData: RawDataExt, 
        E: ExtraExt,
        GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    {
        match kind {
            TabKind::Settings => {
                Self::Settings(Rc::new(get_settings(base.clone())))
            },
            TabKind::Instructions => {
                let callbacks = InstructionsEditorCallbacks::new(clone!(base => move |instructions, also_history| {
                    if(also_history) {
                        base.history.push_modify(|raw| {
                            if let Some(content) = raw.get_content_mut() {
                                content.instructions = instructions;
                            }
                        });
                    } else {
                        base.history.save_current_modify(|raw| {
                            if let Some(content) = raw.get_content_mut() {
                                content.instructions = instructions;
                            }
                        });
                    }
                }));

                let state = InstructionsEditorState::new(base.instructions.clone(), callbacks);

                Self::Instructions(Rc::new(state))
            },
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Settings(_) => TabKind::Settings,
            Self::Instructions(_) => TabKind::Instructions,
        }
    }
}
