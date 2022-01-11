use super::state::*;

use shared::domain::jig::module::body::flashcards::DisplayMode;

impl SidebarSettings {
    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        self.base.extra.settings.display_mode.set_neq(display_mode);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.display_mode = display_mode;
            }
        })
    }

    pub fn toggle_swap(&self) {
        let swap = !self.base.extra.settings.swap.get();

        self.base.extra.settings.swap.set_neq(swap);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.swap = swap;
            }
        })
    }
}
