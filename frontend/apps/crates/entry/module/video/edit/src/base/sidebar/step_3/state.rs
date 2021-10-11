use super::play_settings::state::State as PlaySettingsState;
use crate::base::state::Base;
use components::{
    tabs::MenuTabKind,
    instructions::editor::{callbacks::Callbacks as InstructionsEditorCallbacks, state::State as InstructionsEditorState}
};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::super::state::Sidebar;

pub struct Step3 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        // let kind = match crate::debug::settings().settings_tab {
        //     Some(kind) => kind,
        //     None => MenuTabKind::Settings
        // };

        let kind = MenuTabKind::PlaySettings;

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    Settings(Rc<PlaySettingsState>),
    Instructions(Rc<InstructionsEditorState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::PlaySettings => Self::Settings(Rc::new(PlaySettingsState::new(base))),
            MenuTabKind::Instructions => {
                let callbacks = InstructionsEditorCallbacks::new(
                    clone!(base => move |instructions, also_history| {
                        if also_history {
                            base.history.push_modify(|raw| {
                                if let Some(content) = raw.content.as_mut() {
                                    content.base.instructions = instructions;
                                }
                            });
                        } else {
                            base.history.save_current_modify(|raw| {
                                if let Some(content) = raw.content.as_mut() {
                                    content.base.instructions = instructions;
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
