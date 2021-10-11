use crate::base::state::*;
use std::rc::Rc;
use futures_signals::{
    map_ref, 
    signal::{Mutable, Signal, SignalExt}
};
use dominator::clone;
use components::{
    tabs::MenuTabKind,
    instructions::editor::{
        state::State as InstructionsEditorState,
        callbacks::Callbacks as InstructionsEditorCallbacks
    },
};
use shared::domain::jig::module::body::{Image, Audio};
use std::pin::Pin;
use super::play_settings::state::*;
use super::super::state::Sidebar;

pub struct Step5 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}


impl Step5 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().step_5_tab {
            Some(kind) => kind,
            None => MenuTabKind::PlaySettings
        };
        
        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self {
            sidebar,
            tab
        })
    }

}

#[derive(Clone)]
pub enum Tab {
    Settings(Rc<PlaySettingsState>),
    Instructions(Rc<InstructionsEditorState>),
    Feedback(Rc<InstructionsEditorState>),
}

impl Tab {

    pub fn new(base: Rc<Base>, kind:MenuTabKind) -> Self {
        match kind {
            MenuTabKind::PlaySettings => {
                Self::Settings(Rc::new(PlaySettingsState::new(base)))
            },
            MenuTabKind::Instructions => {
                let callbacks = InstructionsEditorCallbacks::new(clone!(base => move |instructions, also_history| {
                    if(also_history) {
                        base.history.push_modify(|raw| {
                            if let Some(content) = raw.content.as_mut() {
                                content.instructions = instructions;
                            }
                        });
                    } else {
                        base.history.save_current_modify(|raw| {
                            if let Some(content) = raw.content.as_mut() {
                                content.instructions = instructions;
                            }
                        });
                    }
                }));

                let state = InstructionsEditorState::new(base.instructions.clone(), callbacks);

                Self::Instructions(Rc::new(state))
            },
            MenuTabKind::Feedback => {
                let callbacks = InstructionsEditorCallbacks::new(clone!(base => move |instructions, also_history| {
                    if(also_history) {
                        base.history.push_modify(|raw| {
                            if let Some(content) = raw.content.as_mut() {
                                content.feedback = instructions;
                            }
                        });
                    } else {
                        base.history.save_current_modify(|raw| {
                            if let Some(content) = raw.content.as_mut() {
                                content.feedback = instructions;
                            }
                        });
                    }
                }));

                let state = InstructionsEditorState::new(base.instructions.clone(), callbacks);

                Self::Feedback(Rc::new(state))
            },
            _ => unimplemented!("unsupported tab kind!")
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Settings(_) => MenuTabKind::PlaySettings,
            Self::Instructions(_) => MenuTabKind::Instructions,
            Self::Feedback(_) => MenuTabKind::Feedback,
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Self::Settings(_) => 0,
            Self::Instructions(_) => 1,
            Self::Feedback(_) => 2,
        }
    }
}

