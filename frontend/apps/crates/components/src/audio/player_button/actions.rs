use super::state::*;

impl AudioPlayerButton {
    pub fn stop(&self) {
        self.handle.set(None);
    }
}
