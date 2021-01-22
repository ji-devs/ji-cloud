use dominator::clone;
use crate::register::state::{Step, Step2Data};
use serde::Serialize;
use super::state::*;
use std::rc::Rc;
use shared::{
    api::{ApiEndpoint, endpoints},
    domain::{
        meta::{AgeRangeId, AffiliationId},
        auth::{RegisterRequest, RegisterSuccess},
    },
    error::auth::RegisterError
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use utils::{
    storage,
    routes::*,
    fetch::api_with_token
};

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


    let step_2 = state.step_2.clone();
    let step_1 = step_2.step_1;
    let start = step_1.start;

    let req = RegisterRequest {
        username: step_1.username,
        email: start.email,
        over_18: true,
        given_name: step_1.firstname,
        family_name: step_1.lastname,
        language: step_2.language,
        locale: "en".to_string(),
        timezone: chrono_tz::Tz::Asia__Jerusalem,
        opt_into_edu_resources: step_2.marketing,
        organization: None,
        subjects: Vec::new(),
        age_ranges,
        affiliations,
        location: step_2.location_json.map(
            |raw| serde_json::to_value(JsonRaw { raw }).unwrap_throw()
        )
    };


    let token = start.token;
    let email_verified = start.email_verified;

    state.register_loader.load(clone!(state => async move {
        let resp:Result<RegisterSuccess, RegisterError> = api_with_token(&endpoints::user::Register::PATH, &token, endpoints::user::Register::METHOD, Some(req)).await;

        match resp {
            Ok(resp) => match resp {
                //This is confusing... we need to confirm email
                //with both firebase *and* our server??
                RegisterSuccess::Signin(csrf) => {
                    if email_verified {
                        storage::save_csrf_token(&csrf);
                        let route:String = Route::User(UserRoute::RegisterComplete).into();
                        dominator::routing::go_to_url(&route);
                    } else {
                        let route:String = Route::User(UserRoute::SendEmailConfirmation).into();
                        dominator::routing::go_to_url(&route);
                    }
                },
                RegisterSuccess::ConfirmEmail => {
                    let route:String = Route::User(UserRoute::SendEmailConfirmation).into();
                    dominator::routing::go_to_url(&route);
                }

            }, 
            Err(err) => {
                log::error!("unexpected technical error!");
                panic!("{:?}", err);
            }
        }
    }));
}

#[derive(Serialize, Debug)]
struct JsonRaw {
    raw: String
}
