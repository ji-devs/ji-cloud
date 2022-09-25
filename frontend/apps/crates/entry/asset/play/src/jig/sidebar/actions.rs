use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints::{jig::report, meta},
    domain::{
        jig::report::{CreateJigReport, CreateJigReportPath},
        meta::GetMetadataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use crate::jig::sidebar::state::ReportStatus;

use super::state::State;

pub fn send_report(state: Rc<State>) {
    state.player_state.loader.load(clone!(state => async move {
        let report_type = state.report_type.lock_ref().unwrap();

        let response = report::Create::api_with_auth(
            CreateJigReportPath(state.player_state.jig_id),
            Some(CreateJigReport {
                report_type
        })).await;
        if let Ok(_res) = response {
            state.report_status.set(ReportStatus::Sent);
        }
    }));
}

pub fn load_ages(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        match meta::Get::api_no_auth(GetMetadataPath(), None).await {
            Err(_) => {},
            Ok(res) => {
                state.all_ages.set(res.age_ranges);
            },
        }
    }));
}
