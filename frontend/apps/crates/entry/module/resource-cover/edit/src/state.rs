use super::base::{
    actions::init_from_raw, footer::state::Footer, header::state::Header, main::state::Main,
    overlay::state::Overlay, sidebar::state::Sidebar, state::Base,
};
use components::module::_common::edit::prelude::*;
use shared::domain::jig::{
    module::{
        body::resource_cover::{ModuleData as RawData, Step},
        ModuleId,
    },
    JigId,
};
use std::rc::Rc;

pub type AppState = GenericState<(), Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>;

pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new(jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone();
    opts.is_main_scrollable = false;
    opts.skip_save_for_debug = crate::debug::settings().skip_save;
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, init_from_raw)
}
