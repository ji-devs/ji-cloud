use crate::{
    tabs::MenuTabKind,
    instructions::editor::{
        callbacks::Callbacks as InstructionsEditorCallbacks,
        state::State as InstructionsEditorState,
    },
    module::_groups::cards::edit::state::*,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;

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
    pub tab_index: Mutable<Option<usize>>
}

impl<RawData, E, GetSettingsStateFn, SettingsState>
    Step3<RawData, E, GetSettingsStateFn, SettingsState>
where
    RawData: RawDataExt,
    E: ExtraExt,
    GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    SettingsState: 'static,
{
    pub fn new(base: Rc<CardsBase<RawData, E>>, get_settings: GetSettingsStateFn, tab_index: Mutable<Option<usize>>) -> Rc<Self> {
        let kind = match base.debug.step3_tab {
            Some(kind) => kind,
            None => MenuTabKind::PlaySettings,
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind, get_settings.clone()));

        Rc::new(Self {
            base,
            tab,
            get_settings,
            tab_index
        })
    }
}

pub enum Tab<SettingsState> {
    Settings(Rc<SettingsState>),
    Instructions(Rc<InstructionsEditorState>),
}

//the generic makes the auto derive break
impl<SettingsState> Clone for Tab<SettingsState> {
    fn clone(&self) -> Self {
        match self {
            Self::Settings(state) => Self::Settings(state.clone()),
            Self::Instructions(state) => Self::Instructions(state.clone()),
        }
    }
}

impl<SettingsState> Tab<SettingsState> {
    pub fn new<RawData, E, GetSettingsStateFn>(
        base: Rc<CardsBase<RawData, E>>,
        kind: MenuTabKind,
        get_settings: GetSettingsStateFn,
    ) -> Self
    where
        RawData: RawDataExt,
        E: ExtraExt,
        GetSettingsStateFn: Fn(Rc<CardsBase<RawData, E>>) -> SettingsState + Clone + 'static,
    {
        match kind {
            MenuTabKind::PlaySettings => Self::Settings(Rc::new(get_settings(base.clone()))),
            MenuTabKind::Instructions => {
                let callbacks = InstructionsEditorCallbacks::new(
                    clone!(base => move |instructions, also_history| {
                        if also_history {
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
                    }),
                );

                let state = InstructionsEditorState::new(base.instructions.clone(), callbacks);

                Self::Instructions(Rc::new(state))
            },

            _ => unimplemented!("unsupported tab kind!")
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Settings(_) => MenuTabKind::PlaySettings,
            Self::Instructions(_) => MenuTabKind::Instructions,
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            Self::Settings(_) => 0,
            Self::Instructions(_) => 1,
        }
    }
}
