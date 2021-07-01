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

    pub fn set_has_time_limit(&self, flag: bool) {
        self.base.extra.settings.has_time_limit.set_neq(flag);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                if !flag {
                    content.player_settings.time_limit = None; 
                } else {
                    let value = self.base.extra.settings.time_limit.get();
                    content.player_settings.time_limit = Some(value); 
                }
            }
        })
    }
    pub fn set_time_limit(&self, time_limit: u32) {
        self.base.extra.settings.time_limit.set_neq(time_limit);

        if self.base.extra.settings.has_time_limit.get() {
            self.base.history.push_modify(|raw| {
                if let Some(content) = &mut raw.content {
                    content.player_settings.time_limit = Some(time_limit);
                }
            })
        }
    }
}
