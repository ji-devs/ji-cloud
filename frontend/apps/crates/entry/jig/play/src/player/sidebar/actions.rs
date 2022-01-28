use std::rc::Rc;

use dominator::clone;
use shared::{
    api::{
        endpoints::{jig::report, meta},
        ApiEndpoint,
    },
    domain::{
        jig::{report::CreateJigReport, ReportId},
        meta::MetadataResponse,
        CreateResponse,
    },
    error::EmptyError,
};
use utils::prelude::{api_no_auth, api_with_auth};

use crate::player::sidebar::state::ReportStatus;

use super::state::State;

pub fn send_report(state: Rc<State>) {
    state.player_state.loader.load(clone!(state => async move {
        let report_type = state.report_type.lock_ref().unwrap();

        let id = &state.player_state.jig_id.0.to_string();
        let path = report::Create::PATH.replace("{id}", &id);
        let response = api_with_auth::<CreateResponse<ReportId>, EmptyError, CreateJigReport>(
            &path,
            report::Create::METHOD,
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
        match api_no_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
            Err(_) => {},
            Ok(res) => {
                state.all_ages.set(res.age_ranges);
            },
        }
    }));
}
