use components::module::edit::prelude::*;
use super::base::{
    actions::init_from_raw,
    state::Base,
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::tapping_board::{Mode, Step, ModuleData as RawData}}};

pub type AppState = GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>;


pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 
    opts.is_main_scrollable = false;
    opts.skip_save_for_debug = crate::debug::settings().skip_save;
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(
        opts,
        init_from_raw, 
    )
}

