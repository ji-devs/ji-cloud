use super::state::*;
use serde::Serialize;
use shared::{
    api::endpoints,
    domain::{
        meta::{AffiliationId, AgeRangeId, SubjectId},
        user::CreateProfileRequest,
    },
};
use std::rc::Rc;
use utils::{prelude::*, storage};
use uuid::Uuid;

pub fn submit(state: Rc<State>) {
    let age_ranges: Vec<AgeRangeId> = state
        .age_ranges
        .borrow()
        .iter()
        .map(|id| AgeRangeId(Uuid::parse_str(id).unwrap_ji()))
        .collect();

    let affiliations: Vec<AffiliationId> = state
        .affiliations
        .borrow()
        .iter()
        .map(|id| AffiliationId(Uuid::parse_str(id).unwrap_ji()))
        .collect();

    let subjects: Vec<SubjectId> = state
        .subjects
        .borrow()
        .iter()
        .map(|id| SubjectId(Uuid::parse_str(id).unwrap_ji()))
        .collect();

    let step_2 = state.step_2.clone();
    let step_1 = step_2.step_1;

    let req = CreateProfileRequest {
        username: step_1.username,
        over_18: true,
        given_name: step_1.firstname,
        family_name: step_1.lastname,
        language: step_2.language,
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: step_2.marketing,
        organization: step_2.organization,
        persona: step_2.persona,
        profile_image_url: step_1.oauth_profile.and_then(|p| p.profile_picture),
        subjects,
        age_ranges,
        affiliations,
        location: step_2
            .location_json
            .map(|raw| serde_json::to_value(raw).unwrap_ji()),
    };

    state.register_loader.load(async {
        let (resp, status) = endpoints::user::CreateProfile::api_with_auth_status(Some(req)).await;

        match resp {
            Ok(resp) => {
                storage::save_csrf_token(&resp.csrf);
                let route: String = Route::User(UserRoute::RegisterComplete).into();
                dominator::routing::go_to_url(&route);
            }
            Err(err) => {
                let msg = match status {
                    401 => Some(crate::strings::STR_NOT_AUTHORIZED),
                    409 => Some(crate::strings::STR_USER_EXISTS),
                    422 => Some(crate::strings::STR_EMPTY_USERNAME),
                    _ => None,
                };

                if let Some(msg) = msg {
                    let _ = web_sys::window().unwrap_ji().alert_with_message(msg);
                } else {
                    log::error!("unexpected technical error!");
                    panic!("{:?}", err);
                }
            }
        }
    });
}

#[derive(Serialize, Debug)]
struct JsonRaw {
    raw: String,
}
