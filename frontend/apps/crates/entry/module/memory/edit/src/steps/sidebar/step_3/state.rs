use crate::steps::state::{Step, Base};
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
    pub instructions_editor: Rc<InstructionsEditorState>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

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

        let instructions_editor = Rc::new(InstructionsEditorState::new(base.instructions.clone(), callbacks));

        Rc::new(Self {
            base,
            instructions_editor,
        })

    }

}
