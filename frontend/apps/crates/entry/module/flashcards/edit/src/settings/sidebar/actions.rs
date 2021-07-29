use dominator::clone;
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use shared::domain::jig::module::body::flashcards::DisplayMode;
use components::module::_groups::cards::edit::state::RawDataExt;


impl SidebarSettings {
    pub fn set_display_mode(&self, display_mode: DisplayMode) {
        self.base.extra.settings.display_mode.set_neq(display_mode);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.display_mode = display_mode;
            }
        })
    }
}
