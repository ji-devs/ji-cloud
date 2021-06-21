use crate::base::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use dominator::clone;
use components::{
    image::search::state::{State as ImageSearchState, ImageSearchOptions},
    audio_input::{
        options::AudioInputOptions,
        state::State as AudioInputState,
        callbacks::Callbacks as AudioCallbacks,
    },
    stickers::state::Stickers,
};
use dominator_helpers::futures::AsyncLoader;

use shared::domain::jig::module::body::Audio;
use components::{
    instructions::editor::{
        state::State as InstructionsEditorState,
        callbacks::Callbacks as InstructionsEditorCallbacks
    },
};

pub struct Step3 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let kind = match crate::debug::settings().step3_tab {
            Some(kind) => kind,
            None => TabKind::Settings
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab,
        })

    }

}

pub struct SettingsState {
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
    Settings(Rc<SettingsState>),
    Instructions(Rc<InstructionsEditorState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Settings => {
                let state = SettingsState {};

                Self::Settings(Rc::new(state))
            },
            TabKind::Instructions => {
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
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Settings(_) => TabKind::Settings,
            Self::Instructions(_) => TabKind::Instructions,
        }
    }
}
