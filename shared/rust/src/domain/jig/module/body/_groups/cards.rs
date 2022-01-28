/*
 * The card modules not only share some base content
 * But the editor steps are identical except for 3
 */
use crate::{
    config,
    domain::jig::module::body::{
        Audio, Background, Image, Instructions, ModeExt, StepExt, ThemeChoice,
    },
};
use serde::{de, Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// The base content for card modules
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct BaseContent {
    /// The editor state
    pub editor_state: EditorState,

    /// The instructions for the module.
    pub instructions: Instructions,

    /// The mode the module uses.
    pub mode: Mode,

    /// The pairs of cards that make up the module.
    pub pairs: Vec<CardPair>,

    /// The ID of the module's theme.
    pub theme: ThemeChoice,

    /// The optional background override
    pub background: Option<Background>,
}

impl BaseContent {
    /// Get a new BaseContent
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            ..Self::default()
        }
    }

    /// Convenience method to determine whether pairs have been configured correctly
    pub fn is_valid(&self) -> bool {
        let pair_len = self.pairs.len();
        pair_len >= config::MIN_LIST_WORDS
            && pair_len <= config::MAX_LIST_WORDS
            && self.mode.pairs_valid(&self.pairs)
    }
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

/// A pair of cards
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPair(pub Card, pub Card);

/// Data for individual cards
#[derive(Clone, Serialize, Debug)]
pub struct Card {
    /// Recorded audio associated with the card
    pub audio: Option<Audio>,

    /// Content associated with the card
    pub card_content: CardContent,
}

// Required because we need to be able to handle the data for the original Card enum, and also data
// from the new Card struct.
//
// I.e. converts from
//
// [{"Text": "Some words"}, {"Image": {<Image data>}}]
//
// to
//
// [{
//   audio: null,
//   card_content: {"Text": "Some words"}
// }, {
//   audio: null,
//   card_content: {"Image": {<Image data>}}
// }]
//
// TODO Create a content migration to migrate all existing JIGs with card game modules so that
// their card data matches the new Card struct and delete this Deserialize implementation.
impl<'de> de::Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier)]
        enum CardField {
            #[serde(rename = "audio")]
            Audio,
            #[serde(rename = "card_content")]
            CardContent,
            #[serde(rename = "Text")]
            Text,
            #[serde(rename = "Image")]
            Image,
        }

        struct CardVisitor;

        impl<'de> de::Visitor<'de> for CardVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("A CardContent or Card map")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut audio: Option<Option<Audio>> = None;
                let mut card_content: Option<CardContent> = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        CardField::Text => {
                            if card_content.is_some() {
                                return Err(de::Error::duplicate_field("card_content"));
                            }
                            card_content = Some(CardContent::Text(access.next_value()?));
                            break;
                        }
                        CardField::Image => {
                            if card_content.is_some() {
                                return Err(de::Error::duplicate_field("card_content"));
                            }
                            card_content = Some(CardContent::Image(access.next_value()?));
                            break;
                        }
                        CardField::Audio => {
                            if audio.is_some() {
                                return Err(de::Error::duplicate_field("audio"));
                            }
                            audio = Some(access.next_value()?);
                        }
                        CardField::CardContent => {
                            if card_content.is_some() {
                                return Err(de::Error::duplicate_field("card_content"));
                            }
                            card_content = Some(access.next_value()?);
                        }
                    }
                }

                let audio = audio.map_or(None, |audio| audio);
                let card_content =
                    card_content.ok_or_else(|| de::Error::missing_field("card_content"))?;

                Ok(Card {
                    audio,
                    card_content,
                })
            }
        }

        deserializer.deserialize_map(CardVisitor)
    }
}
/// The content of a card
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CardContent {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Text(String),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Image(Option<Image>),
}

impl Card {
    /// Whether the variants value is empty
    pub fn is_empty(&self) -> bool {
        match &self.card_content {
            CardContent::Text(value) if value.trim().len() == 0 => true,
            CardContent::Image(None) => true,
            _ => false,
        }
    }
}

/// What mode the module runs in.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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
    Synonyms = 6,

    /// Translate from one language to another.
    Translate = 7,
}

impl Mode {
    /// Returns whether a list of card pairs are valid for the game mode
    pub fn pairs_valid(&self, pairs: &Vec<CardPair>) -> bool {
        match self {
            // Text/Image pairs
            Self::WordsAndImages => {
                pairs
                    .iter()
                    .find(|pair| {
                        // Neither card should be empty; the first card should be a Text variant and
                        // the 2nd card should be an Image variant.
                        pair.0.is_empty()
                            || pair.1.is_empty()
                            || !matches!(pair.0.card_content, CardContent::Text(_))
                            || !matches!(pair.1.card_content, CardContent::Image(_))
                    })
                    .is_none()
            }
            // Text/Text pairs
            _ => {
                pairs
                    .iter()
                    .find(|pair| {
                        // Neither card should be empty, and both cards must be Image variants.
                        pair.0.is_empty()
                            || pair.1.is_empty()
                            || !matches!(pair.0.card_content, CardContent::Text(_))
                            || !matches!(pair.1.card_content, CardContent::Text(_))
                    })
                    .is_none()
            }
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Duplicate
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Duplicate,
            Self::WordsAndImages,
            Self::BeginsWith,
            Self::Lettering,
            Self::Riddles,
            Self::Opposites,
            Self::Synonyms,
            Self::Translate,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::WordsAndImages => "words-images",
            Self::BeginsWith => "begins-with",
            Self::Lettering => "lettering",
            Self::Riddles => "riddles",
            Self::Opposites => "opposites",
            Self::Synonyms => "synonyms",
            Self::Translate => "translate",
        }
    }

    fn label(&self) -> &'static str {
        const STR_DUPLICATE: &'static str = "Duplicate";
        const STR_WORDS_IMAGES: &'static str = "Words & Images";
        const STR_BEGINS_WITH: &'static str = "What begins with...";
        const STR_LETTERING: &'static str = "Lettering";
        const STR_RIDDLES: &'static str = "Riddles";
        const STR_OPPOSITES: &'static str = "Opposites";
        const STR_SYNONYMS: &'static str = "Synonyms";
        const STR_TRANSLATE: &'static str = "Translate";

        match self {
            Self::Duplicate => STR_DUPLICATE,
            Self::WordsAndImages => STR_WORDS_IMAGES,
            Self::BeginsWith => STR_BEGINS_WITH,
            Self::Lettering => STR_LETTERING,
            Self::Riddles => STR_RIDDLES,
            Self::Opposites => STR_OPPOSITES,
            Self::Synonyms => STR_SYNONYMS,
            Self::Translate => STR_TRANSLATE,
        }
    }
}

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    /// Step 1
    One,
    /// Step 2
    Two,
    /// Step 3
    Three,
    /// Step 4
    Four,
}

impl Default for Step {
    fn default() -> Self {
        Self::One
    }
}

impl StepExt for Step {
    fn next(&self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_CONTENT: &'static str = "Content";
        const STR_DESIGN: &'static str = "Design";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_CONTENT,
            Self::Two => STR_DESIGN,
            Self::Three => STR_SETTINGS,
            Self::Four => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}
