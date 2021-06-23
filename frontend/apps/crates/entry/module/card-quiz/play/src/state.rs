use components::module::play::prelude::*;
use super::base::state::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    module::{
        ModuleId, 
        body::{
            _groups::cards::{Mode, Step},
            card_quiz::ModuleData as RawData
        }
    }
};

pub type AppState = GenericState<RawData, Mode, Step, Base>;


pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, Base::new)
}
