use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints::jig}, domain::jig::{JigPlayerSettings, player::{JigPlayerSessionCode, JigPlayerSessionCreateRequest}}, error::EmptyError};
use utils::prelude::*;

use super::state::State;

pub(super) fn generate_student_code(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let req = shared::domain::jig::player::JigPlayerSessionCreateRequest {
                jig_id: state.jig_id.clone(),
                settings: JigPlayerSettings::default(),
            };

        match api_with_auth::<JigPlayerSessionCode, EmptyError, JigPlayerSessionCreateRequest>(
            jig::player::Create::PATH, jig::player::Create::METHOD, Some(req)
        ).await {
            Err(_) => todo!(),
            Ok(res) => {
                // let code = res.index.to_string();
                let code = format!("{:04}", res.index);
                state.student_code.set(Some(code));
            },
        };

    }));
}
