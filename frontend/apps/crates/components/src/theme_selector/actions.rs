use super::state::*;
use shared::domain::jig::module::body::ThemeId;

impl ThemeSelector {
    pub fn set_theme(&self, theme: ThemeId) {
        (self.callbacks.on_change)(theme);
    }
}
