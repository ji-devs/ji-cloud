use std::rc::Rc;

use dominator::clone;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

use crate::player::sidebar::state::ReportStatus;

use super::state::State;


pub fn send_report(state: Rc<State>) {
    state.player_state.loader.load(clone!(state => async move {
        let report_type = state.report_type.lock_ref();
        log::info!("Sending report: {:?}", report_type);
        // TODO: actually send report
        state.report_status.set(ReportStatus::Sent);

        spawn_local(clone!(state => async move {
            TimeoutFuture::new(5_000).await;
            let mut report_status = state.report_status.lock_mut();
            // only update if status hasn't changed
            if *report_status == ReportStatus::Sent {
                *report_status = ReportStatus::Default;
            }
        }));
    }));
}
