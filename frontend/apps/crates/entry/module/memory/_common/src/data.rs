use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub mode: Mode,
    pub pairs: Vec<(Card, Card)>,
    pub theme_id: String,
}

#[derive(Serialize, Deserialize,Clone, Debug)]
pub enum Mode {
    Duplicate,
    WordsAndImages
}

impl GameData {
    pub async fn load(jig_id:String, module_id:String) -> Option<Self> {
        //TODO - load
        None
    }

    pub fn duplicate_debug<I, S>(words:I, theme_id: String) -> Self 
        where I: Iterator<Item = S>,
              S: AsRef<str>
    {
        Self {
            mode: Mode::Duplicate,
            pairs: words
                .map(|word| {
                    let word = word.as_ref();
                    (Card::new_text(word.to_string()), Card::new_text(word.to_string()))
                })
                .collect(),
            theme_id
        }
    }
    pub fn words_and_images_debug<I, S>(words:I, theme_id: String) -> Self 
        where I: Iterator<Item = S>,
              S: AsRef<str>
    {
        Self {
            mode: Mode::WordsAndImages,
            pairs: words
                .map(|word| {
                    let word = word.as_ref();
                    (Card::new_text(word.to_string()), Card::new_image(None))
                })
                .collect(),
            theme_id
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Card {
    Text(String),
    Image(Option<String>),
    Audio(Option<String>)
}

impl Card {
    pub fn new_text(text:String) -> Self {
        Card::Text(text)
    }
    pub fn new_image(src:Option<String>) -> Self {
        Card::Image(src)
    }
    pub fn new_audio(src:Option<String>) -> Self {
        Card::Audio(src)
    }
}
