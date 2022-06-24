use super::base::state::*;
use components::module::_common::play::prelude::*;
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            _groups::cards::{Mode, Step},
            memory::ModuleData as RawData,
        },
        ModuleId,
    },
};
use std::rc::Rc;

pub type AppState = GenericState<RawData, Mode, Step, Base>;

pub fn create_state(asset_id: AssetId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(asset_id, module_id);

    let mut opts = StateOpts::new(asset_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone();
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(opts, Base::new)
}
