use super::base::{
    actions::init_from_raw, footer::state::Footer, header::state::Header, main::state::Main,
    overlay::state::Overlay, sidebar::state::Sidebar, state::Base,
};
use components::module::_common::edit::prelude::*;
use shared::domain::{
    asset::AssetId,
    module::{
        body::video::{Mode, ModuleData as RawData, Step},
        ModuleId,
    },
};
use std::rc::Rc;

pub type AppState = GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>;

pub fn create_state(asset_id: AssetId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(asset_id, module_id);

    let mut opts = StateOpts::new(asset_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone();
    opts.is_main_scrollable = false;
    opts.skip_save_for_debug = crate::debug::settings().skip_save;
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, init_from_raw)
}
