use components::module::play::state::*;
use super::main::state::*;
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::tapping_board::{Mode as RawMode, ModuleData as RawData}}};

pub type AppState = GenericState<RawData, Main>;


pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 

    AppState::new(opts, Main::new)
}

