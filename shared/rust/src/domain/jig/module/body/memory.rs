use crate::{
    domain::{audio::AudioId, image::ImageId, jig::module::theme::ThemeId},
    media::MediaLibrary,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// A pair of cards
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CardPair(pub Card, pub Card);

/// The body for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    pub instructions: Instructions,
    pub mode: Option<Mode>,
    pub pairs: Vec<CardPair>,
    pub theme_id: ThemeId,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Card {
    Text(String),
    Image(Option<(ImageId, MediaLibrary)>),
    Audio(Option<(AudioId, MediaLibrary)>),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum Mode {
    Duplicate = 0,
    WordsAndImages = 1,
    BeginsWith = 2,
    Lettering = 3,
    Riddles = 4,
    Opposites = 5,
    Synonymns = 6,
    Translate = 7,
}

impl Mode {
    //Must match the element strings in types
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::WordsAndImages => "words-images",
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
        S: AsRef<str>,
    {
        Self {
            mode: Some(mode),
            instructions,
            pairs: pairs
                .into_iter()
                .map(|(word_1, word_2)| {
                    let (word_1, word_2) = (word_1.as_ref(), word_2.as_ref());

                    match mode {
                        Mode::WordsAndImages => {
                            CardPair(Card::Text(word_1.to_string()), Card::Image(None))
                        }
                        _ => CardPair(
                            Card::Text(word_1.to_string()),
                            Card::Text(word_2.to_string()),
                        ),
                    }
                })
                .collect(),
            theme_id,
        }
    }
}
