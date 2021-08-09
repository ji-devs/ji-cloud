use std::rc::Rc;

use dominator::clone;
use gloo_timers::future::TimeoutFuture;
use shared::{
    api::{endpoints::meta, ApiEndpoint},
    domain::meta::MetadataResponse,
    error::EmptyError,
};
use utils::prelude::api_no_auth;
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

pub fn load_ages(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        match api_no_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
            Err(_) => {},
            Ok(res) => {
                state.all_ages.set(res.age_ranges);
            },
        }
    }));
}
