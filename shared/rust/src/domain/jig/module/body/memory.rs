#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;
use crate::{
    media::MediaLibrary,
    domain::{
        audio::AudioId,
        image::ImageId,
        jig::module::theme::ThemeId
    }
};

/// The body for [`Memory`](super::ModuleKind::Memory) modules.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
pub struct ModuleData {
    pub instructions: Instructions,
    pub mode: Mode,
    pub pairs: Vec<(Card, Card)>,
    pub theme_id: ThemeId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Instructions {
    pub text: Option<String>,
    pub audio_id: Option<AudioId>,
}

impl Instructions {
    pub fn new() -> Self {
        Self { 
            text: None,
            audio_id: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Card {
    Text(String),
    Image(Option<(ImageId, MediaLibrary)>),
    Audio(Option<(AudioId, MediaLibrary)>)
}



#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Mode {
    Duplicate,
    WordsAndImages,
    BeginsWith,
    Lettering,
    Riddles,
    Opposites,
    Synonymns,
    Translate
}

impl Mode {
    //Must match the element strings in types
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::WordsAndImages=> "words-images",
            Self::BeginsWith => "begins-with",
            Self::Lettering => "lettering",
            Self::Riddles => "riddles",
            Self::Opposites => "opposites",
            Self::Synonymns => "synonymns",
            Self::Translate => "translate",
        }
    }
}

impl ModuleData {
    pub fn new<I, S>(mode: Mode, theme_id: ThemeId, instructions: Instructions, pairs: I) -> Self 
        where 
            I: IntoIterator<Item = (S, S)>,
            S: AsRef<str>
    {
        Self {
            mode,
            instructions,
            pairs: pairs 
                .into_iter()
                .map(|(word_1, word_2)| {
                    let (word_1, word_2) = (word_1.as_ref(), word_2.as_ref());

                    match mode {
                        Mode::WordsAndImages => {
                            (Card::Text(word_1.to_string()), Card::Image(None))
                        },
                        _ => {
                            (Card::Text(word_1.to_string()), Card::Text(word_2.to_string()))
                        },
                    }
                })
                .collect(),
            theme_id,
        }
    }
}

