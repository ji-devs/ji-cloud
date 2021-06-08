use super::{
    state::*,
    super::{
        state::Phase,
        playing::state::PlayState
    }
};

impl Hints {
    pub fn finish(&self) {
        self.game.phase.set_neq(Phase::Playing);
    }
}

