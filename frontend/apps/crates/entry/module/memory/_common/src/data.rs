use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameData {
    pub mode: Mode,
    pub pairs: Vec<(Card, Card)>,
    pub theme: String,
}

#[derive(Serialize, Deserialize,Clone, Debug)]
pub enum Mode {
    Duplicate,
    WordsAndImages
}

impl GameData {
    pub async fn load(jig_id:String, module_id:String) -> Result<Self, ()> {
        //TODO - load
        Err(())
    }
    pub fn new_duplicate() -> Self 
    {
        Self {
            mode: Mode::Duplicate,
            pairs: Vec::new(), 
            theme: "".to_string()
        }
    }

    pub fn duplicate_debug<I, S>(words:I, theme: String) -> Self 
        where I: Iterator<Item = S>,
              S: AsRef<str>
    {
        Self {
            mode: Mode::Duplicate,
            pairs: words
                .map(|word| {
                    let word = word.as_ref();
                    (Card::Text(Some(word.to_string())), Card::Text(Some(word.to_string())))
                })
                .collect(),
            theme
        }
    }
    pub fn words_and_images_debug<I, S>(words:I, theme: String) -> Self 
        where I: Iterator<Item = S>,
              S: AsRef<str>
    {
        Self {
            mode: Mode::WordsAndImages,
            pairs: words
                .map(|word| {
                    let word = word.as_ref();
                    (Card::Text(Some(word.to_string())), Card::Image(None))
                })
                .collect(),
            theme
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Card {
    Text(Option<String>),
    Image(Option<String>),
    Audio(Option<String>)
}
