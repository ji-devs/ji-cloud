use super::{raw, state};
use serde::{Serialize, Deserialize};
use shared::{
    domain::image::ImageId,
    domain::audio::AudioId,
    media::MediaLibrary
};

//History's game data differs from:
//raw: content can be optional
//state: no mutables

#[derive(Clone, Debug)]
pub struct History {
    pub game_data: Option<raw::GameData>
}

impl History {
    pub fn new(game_data:Option<raw::GameData>) -> Self {
        Self {
            //set initial history to game state?
            game_data
        }
    }
}

