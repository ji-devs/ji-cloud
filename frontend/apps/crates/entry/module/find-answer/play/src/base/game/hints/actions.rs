use super::{super::state::Phase, state::*};

impl Hints {
    pub fn finish(&self) {
        self.game.phase.set_neq(Phase::Playing);
    }
}
