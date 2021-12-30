use super::base::state::*;
use components::module::_common::play::prelude::*;
use shared::domain::jig::{
    module::{
        body::resource_cover::{ModuleData as RawData, Step},
        ModuleId,
    },
    JigId,
};
use std::rc::Rc;
pub type AppState = GenericState<RawData, (), Step, Base>;

pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new(jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone();
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, Base::new)
}
