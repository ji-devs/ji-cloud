use super::state::*;

use shared::domain::module::body::flashcards::DisplayMode;

impl SidebarSettings {
    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        self.base.extra.settings.display_mode.set_neq(display_mode);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.display_mode = display_mode;
            }
        })
    }

    pub fn set_view_pairs(&self, view_pairs: u32) {
        let total_pairs = self.base.pairs.lock_ref().len() as u32;
        let view_pairs = total_pairs.min(view_pairs);

        self.base.extra.settings.view_pairs.set_neq(view_pairs);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.view_pairs = Some(view_pairs);
            }
        })
    }

    pub fn set_view_all(&self, view_all: bool) {
        self.base.extra.settings.view_all.set_neq(view_all);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.view_pairs = None;
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
