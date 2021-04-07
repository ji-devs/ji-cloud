use serde::{Serialize, Deserialize};
use shared::{
    domain::image::ImageId,
    domain::audio::AudioId,
    media::MediaLibrary
};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameData {
    pub instructions: Instructions,
    pub mode: Mode,
    pub pairs: Vec<(Card, Card)>,
    pub theme: String,
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

impl GameData {
    pub fn new<I, S>(mode: Mode, theme: String, instructions: Instructions, pairs: I) -> Self 
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
                        _ => unimplemented!("TODO")
                    }
                })
                .collect(),
            theme
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

