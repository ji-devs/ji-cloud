use crate::{
    domain::{
        audio::AudioId,
        image::ImageId,
        jig::module::body::{Instructions, ThemeChoice, ThemeId},
    },
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
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The mode the module uses.
    pub mode: Option<Mode>,

    /// The pairs of cards that make up the module.
    pub pairs: Vec<CardPair>,

    /// The ID of the module's theme.
    pub theme: ThemeChoice,
}

/// An individual card.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Card {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Text(String),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Image(Option<(ImageId, MediaLibrary)>),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Audio(Option<(AudioId, MediaLibrary)>),
}

/// What mode the module runs in.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum Mode {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Duplicate = 0,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    WordsAndImages = 1,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    BeginsWith = 2,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Lettering = 3,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Riddles = 4,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Opposites = 5,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Synonymns = 6,

    /// Translate from one language to another.
    Translate = 7,
}

impl Mode {
    //Must match the element strings in types
    /// Converts `self` to a [`str`].
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
    /// Instantiates a new module with the given `mode`, `theme_id`, `instructions`, and pairs of text cards.
    pub fn new<I, S>(mode: Mode, theme: ThemeChoice, instructions: Instructions, pairs: I) -> Self
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
            theme,
        }
    }
}
