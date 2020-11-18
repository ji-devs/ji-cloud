use serde::{Serialize, Deserialize};
use crate::config;


#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateRaw {
    pub mode: Option<GameModeRaw>,
    pub mode_state: Option<ModeStateRaw> 
}

impl GameStateRaw {
    pub async fn load() -> Self {

        Self {
            mode: None, 
            mode_state: None 
        }
    }

    pub fn debug() -> Self {
        Self {
            mode: Some(GameModeRaw::Duplicate),
            mode_state: Some(ModeStateRaw::Duplicate(DuplicateStateRaw::debug())),
        }
    }

}

impl From<DuplicateStateRaw> for GameStateRaw {
    fn from(state:DuplicateStateRaw) -> Self {
        Self {
            mode: Some(GameModeRaw::Duplicate),
            mode_state: Some(ModeStateRaw::Duplicate(state))
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum GameModeRaw {
    Duplicate
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModeStateRaw {
    Duplicate(DuplicateStateRaw)
}

#[derive(Serialize, Deserialize, Debug)]
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
pub struct DuplicateStateRaw {
    pub cards: Vec<CardRaw>,
    pub theme_id: String,
}

impl DuplicateStateRaw {
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
