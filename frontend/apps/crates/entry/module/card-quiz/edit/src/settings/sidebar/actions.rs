use dominator::clone;
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use components::module::_groups::cards::edit::state::RawDataExt;


impl SidebarSettings {
    pub fn set_n_choices(&self, n_choices: u8) {
        self.base.extra.settings.n_choices.set_neq(n_choices);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.n_choices = n_choices;
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
