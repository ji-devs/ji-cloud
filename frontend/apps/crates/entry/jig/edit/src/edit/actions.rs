use super::state::*;
use utils::{storage, unwrap::UnwrapJiExt};

impl State {
    pub fn set_permanently_closed(&self) {
        let _ = storage::get_local_storage()
            .unwrap_ji()
            .set_item("onboarding", "hidden");
        self.show_onboarding.set_neq(false);
    }
}
