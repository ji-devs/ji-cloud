use super::raw::*;

#[derive(Clone, Debug)]
pub struct History {
    pub game_data: Option<GameData>
}

impl History {
    pub fn new(game_data: Option<GameData>) -> Self {
        Self {
            game_data
        }
    }
}
