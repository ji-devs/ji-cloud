use crate::base::state::Base;
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use components::{
    instructions::editor::{
        state::State as InstructionsEditorState,
        callbacks::Callbacks as InstructionsEditorCallbacks
    },
};
use shared::domain::jig::module::body::{Image, Audio};
use super::play_settings::{
    state::State as PlaySettingsState,
    dom::render as render_play_settings
};
pub struct Step4 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step4 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let kind = match crate::debug::settings().settings_tab {
            Some(kind) => kind,
            None => TabKind::Settings
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
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

#[derive(Clone)]
pub enum Tab {
    Settings(Rc<PlaySettingsState>),
    Instructions(Rc<InstructionsEditorState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Settings => {
                Self::Settings(Rc::new(PlaySettingsState::new(base)))
            },
            TabKind::Instructions => {
                let callbacks = InstructionsEditorCallbacks::new(clone!(base => move |instructions, also_history| {
                    if(also_history) {
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
                }));

                let state = InstructionsEditorState::new(base.instructions.clone(), callbacks);

                Self::Instructions(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Settings(_) => TabKind::Settings,
            Self::Instructions(_) => TabKind::Instructions,
        }
    }
}

