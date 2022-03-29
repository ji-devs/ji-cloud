use std::rc::Rc;

use dominator::clone;
use gloo_timers::future::TimeoutFuture;
use utils::{storage, unwrap::UnwrapJiExt};
use wasm_bindgen_futures::spawn_local;

use super::JigziHelp;

pub(super) const INFO_TOOLTIP_DELAY: u32 = 1_500;

impl JigziHelp {
    pub(super) fn show_info_tooltip_delayed(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            TimeoutFuture::new(INFO_TOOLTIP_DELAY).await;
            state.show_info_tooltip.set(true);
        }));
    }

    pub(super) fn permanently_close(self: &Rc<Self>) {
        let state = self;

        if state.show_id != "debug" {
            self.permanently_closed.set(true);

            let _ = storage::get_local_storage()
                .unwrap_ji()
                .set_item(&format!("tooltip-{}", state.show_id), "hidden");
        }
    }
}
