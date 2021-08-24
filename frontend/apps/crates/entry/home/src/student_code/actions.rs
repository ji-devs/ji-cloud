use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints::jig}, domain::jig::player::JigPlayerSession, error::EmptyError};
use utils::{prelude::api_no_auth_status, routes::{JigRoute, Route}};

use super::state::State;

pub fn submit_code(state: Rc<State>, number: String) {
    state.loader.load(clone!(state => async move {
        let path = jig::player::Get::PATH.replace("{index}", &number);

        let (result, status) = api_no_auth_status::<JigPlayerSession, EmptyError, ()>(&path, jig::player::Get::METHOD, None).await;

        match status {
            404 => {
                state.error.set(true);
            },
            _ => match result {
                Err(_) => {}
                Ok(res) => {
                    state.error.set_neq(false);
                    state.play_jig.set(Some((res.jig_id, res.settings)));
                }
            },
        };


        // utils::prelude::api_with_auth::<shared::domain::jig::player::JigPlayerSessionCode,EmptyError,shared::domain::jig::player::JigPlayerSessionCreateRequest>(
        //     jig::player::Create::PATH,
        //     jig::player::Create::METHOD,
        //     Some(shared::domain::jig::player::JigPlayerSessionCreateRequest {
        //         jig_id: shared::domain::jig::JigId(uuid::Uuid::from_str("dea981c6-fac4-11eb-9876-b78e9706b646").unwrap()),
        //         settings: shared::domain::jig::JigPlayerSettings::default(),
        //     })
        // ).await;

    }));
}
