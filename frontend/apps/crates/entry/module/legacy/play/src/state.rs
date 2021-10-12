use components::module::_common::play::prelude::*;
use super::base::state::*;
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::legacy::{ModuleData as RawData}}};
pub type AppState = GenericState<RawData, (), (), Base>;


pub async fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id).await;

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, Base::new)
}

