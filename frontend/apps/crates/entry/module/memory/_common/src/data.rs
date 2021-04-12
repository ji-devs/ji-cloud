use serde::{Serialize, Deserialize};
use shared::{
    domain::image::ImageId,
    domain::audio::AudioId,
    media::MediaLibrary
};
use utils::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameData {
    pub instructions: Instructions,
    pub mode: Mode,
    pub pairs: Vec<(Card, Card)>,
    pub theme_id: ThemeId,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Card {
    Text(String),
    Image(Option<(ImageId, MediaLibrary)>),
    Audio(Option<(AudioId, MediaLibrary)>)
}



#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
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
    //Must match the element stings in types.ts
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

impl GameData {
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
                        Mode::Duplicate | Mode::Lettering => {
                            (Card::Text(word_1.to_string()), Card::Text(word_2.to_string()))
                        },
                        Mode::WordsAndImages => {
                            (Card::Text(word_1.to_string()), Card::Image(None))
                        },
                        _ => unimplemented!("TODO")
                    }
                })
                .collect(),
            theme_id,
        }
    }

    /*
    pub fn words_and_images_debug<I, S>(words:I, theme: String) -> Self 
        where I: Iterator<Item = S>,
              S: AsRef<str>
    {
        Self {
            mode: Mode::WordsAndImages,
            pairs: words
                .map(|word| {
                    let word = word.as_ref();
                    (Card::Text(word.to_string()), Card::Image(None))
                })
                .collect(),
            theme
        }
    }
    */
}

