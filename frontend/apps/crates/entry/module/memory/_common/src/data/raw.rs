use serde::{Serialize, Deserialize};
use crate::config;


#[derive(Serialize, Deserialize, Debug)]
pub enum GameStateRaw {
    None,
    Duplicate(BaseGameStateRaw),
    WordsAndImages(BaseGameStateRaw)
}

impl GameStateRaw {
    pub async fn load() -> Self {
        Self::None
    }

    pub fn debug() -> Self {
        Self::Duplicate(BaseGameStateRaw::debug())
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardRaw {
    pub text: String,
}
impl CardRaw {
    pub fn new(text:String) -> Self {
        Self {
            text 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseGameStateRaw {
    pub cards: Vec<CardRaw>,
    pub theme_id: String,
}

impl BaseGameStateRaw {
    pub fn default() -> Self {
        Self {
            cards: config::INITIAL_CARD_TEXTS
                .iter()
                .map(|text| {
                    CardRaw::new(text.to_string())
                })
                .collect(),
            theme_id: config::THEME_OPTIONS[0].id.to_string(), 
        }
    }
    pub fn debug() -> Self {
        Self {
            cards: config::DEBUG_PLAY_CARD_TEXTS
                .iter()
                .map(|text| {
                    CardRaw::new(text.to_string())
                })
                .collect(),
            theme_id: config::THEME_OPTIONS[config::DEBUG_THEME_INDEX].id.to_string(), 
        }
    }
}
