use shared::domain::jig::{JigId, module::{ModuleId, body::tapping_board::{Mode as RawMode, ModuleData as RawData}}};
use components::module::play::state::MainExt;
use utils::prelude::*;

pub struct Main {
    pub text: String
}

impl Main {
    pub async fn new(jig_id: JigId, module_id: ModuleId, raw:RawData, ) -> Self {

        Self {
            text: serde_json::to_string(&raw).unwrap_ji()
        }
    }
}

impl MainExt for Main {
}
