use dominator::clone;
use crate::register::state::{Step, Step2Data};
use serde::Serialize;
use super::state::*;
use std::rc::Rc;
use shared::{
    api::{ApiEndpoint, endpoints},
    domain::{
        meta::{AgeRangeId, AffiliationId, SubjectId},
        user::PutProfileRequest,
        session::NewSessionResponse,
    },
    error::EmptyError,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use utils::{
    storage,
    prelude::*,
    api_helpers::meta::MetaOptions
};

impl State {
    pub fn pre_select(&self, meta:&MetaOptions) {
        let affiliations = &mut *self.affiliations.borrow_mut();
        let age_ranges = &mut *self.age_ranges.borrow_mut();

        for (id, _) in meta.affiliations.iter() {
            affiliations.insert(id.clone());    
        }

        for (id, _) in meta.age_ranges.iter() {
            age_ranges.insert(id.clone());    
        }
    }
}

pub fn submit(state: Rc<State>) {

    let age_ranges:Vec<AgeRangeId> = state.age_ranges
        .borrow()
        .iter()
        .map(|id| {
            AgeRangeId(Uuid::parse_str(id).unwrap_throw())
        })
        .collect();

    let affiliations:Vec<AffiliationId> = state.affiliations
        .borrow()
        .iter()
        .map(|id| {
            AffiliationId(Uuid::parse_str(id).unwrap_throw())
        })
        .collect();

    let subjects:Vec<SubjectId> = state.subjects
        .borrow()
        .iter()
        .map(|id| {
            SubjectId(Uuid::parse_str(id).unwrap_throw())
        })
        .collect();

    let step_2 = state.step_2.clone();
    let step_1 = step_2.step_1;

    let req = PutProfileRequest {
        username: step_1.username,
        over_18: true,
        given_name: step_1.firstname,
        family_name: step_1.lastname,
        language: step_2.language,
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: step_2.marketing,
        organization: Some(step_2.organization),
        persona: Some(step_2.persona),
        //TODO: bring back when https://github.com/ji-devs/ji-cloud/issues/1513 is resolved
        //profile_image: step_1.oauth_profile.and_then(|p| p.profile_picture),
        profile_image_id: None,
        subjects,
        age_ranges,
        affiliations,
        location: step_2.location_json.map(
            |raw| serde_json::to_value(JsonRaw { raw }).unwrap_throw()
        )
    };

    log::info!("{:?}", req);


    state.register_loader.load(clone!(state => async move {
        let (resp, status):(Result<NewSessionResponse, EmptyError>, u16) = api_with_auth_status(&endpoints::user::PutProfile::PATH, endpoints::user::PutProfile::METHOD, Some(req)).await;

        match resp {
            Ok(resp) => {
                storage::save_csrf_token(&resp.csrf);
                let route:String = Route::User(UserRoute::RegisterComplete).into();
                dominator::routing::go_to_url(&route);
            }, 
            Err(err) => {
                let msg = match status {
                    401 => Some(crate::strings::STR_NOT_AUTHORIZED),
                    409 => Some(crate::strings::STR_USER_EXISTS),
                    422 => Some(crate::strings::STR_EMPTY_USERNAME),
                    _ => None
                };

                if let Some(msg) = msg {
                    web_sys::window().unwrap_throw().alert_with_message(msg);
                } else {
                    log::error!("unexpected technical error!");
                    panic!("{:?}", err);
                }
            }
        }
    }));
}

#[derive(Serialize, Debug)]
struct JsonRaw {
    raw: String
}
