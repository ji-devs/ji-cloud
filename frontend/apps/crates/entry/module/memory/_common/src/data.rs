use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GameState {
    None,
    Duplicate(BaseGameState),
    WordsAndImages(BaseGameState)
}

impl GameState {
    pub async fn load(jig_id:String, module_id:String) -> Self {
        //TODO - load
        Self::None
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
    pub fn new_img(src:Option<String>) -> Self {
        Card::Image(src)
    }
    pub fn new_audio(src:Option<String>) -> Self {
        Card::Audio(src)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseGameState {
    pub pairs: Vec<(Card, Card)>,
    pub theme_id: String,
}
