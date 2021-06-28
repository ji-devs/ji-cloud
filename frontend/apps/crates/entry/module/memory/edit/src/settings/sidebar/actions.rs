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
    pub fn set_time_limit(&self, time_limit: Option<u32>) {
        self.base.extra.settings.time_limit.set_neq(time_limit);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.time_limit = time_limit;
            }
        })
    }
}
