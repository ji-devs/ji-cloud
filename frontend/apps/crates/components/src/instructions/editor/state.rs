use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::module::body::{Audio, Instructions};

use super::callbacks::Callbacks;

pub const STR_LABEL_INSTRUCTIONS: &str = "Written instructions";
pub const STR_PLACEHOLDER_INSTRUCTIONS: &str = "Type instructions";
pub const STR_LABEL_FEEDBACK: &str = "Written feedback";
pub const STR_PLACEHOLDER_FEEDBACK: &str = "Type feedback";

pub struct State {
    pub instructions: Mutable<Instructions>,
    pub callbacks: Callbacks,
    pub instructions_text: InstructionsText,
}

pub enum InstructionsType {
    Instructions,
    Feedback,
    Custom(InstructionsText),
}

pub struct InstructionsText {
    pub label: &'static str,
    pub placeholder: &'static str,
}

impl From<InstructionsType> for InstructionsText {
    fn from(instructions_type: InstructionsType) -> Self {
        match instructions_type {
            InstructionsType::Instructions => Self {
                label: STR_LABEL_INSTRUCTIONS,
                placeholder: STR_PLACEHOLDER_INSTRUCTIONS,
            },
            InstructionsType::Feedback => Self {
                label: STR_LABEL_FEEDBACK,
                placeholder: STR_PLACEHOLDER_FEEDBACK,
            },
            InstructionsType::Custom(instructions_text) => instructions_text,
        }
    }
}

impl State {
    pub fn new(
        instructions: Mutable<Instructions>,
        callbacks: Callbacks,
        instructions_type: InstructionsType,
    ) -> Self {
        Self {
            instructions,
            callbacks,
            instructions_text: instructions_type.into(),
        }
    }

    pub fn text_signal(&self) -> impl Signal<Item = String> {
        self.instructions
            .signal_cloned()
            .map(|instructions| match instructions.text {
                None => "".to_string(),
                Some(text) => text,
            })
    }
    pub fn audio_signal(&self) -> impl Signal<Item = Option<Audio>> {
        self.instructions
            .signal_cloned()
            .map(|instructions| instructions.audio)
    }
}
