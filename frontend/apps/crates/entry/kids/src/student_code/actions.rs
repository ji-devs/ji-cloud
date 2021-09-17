use std::rc::Rc;

use dominator::clone;
use shared::{api::{ApiEndpoint, endpoints::jig}, domain::jig::player::JigPlayerSession, error::EmptyError};
use utils::prelude::*;

use super::state::State;

pub fn submit_code(state: Rc<State>, number: String) {
    state.loader.load(clone!(state => async move {
        panic!("fix needed");
        let path = jig::player::instance::Create::PATH.replace("{index}", &number);

        let (result, status) = api_no_auth_status::<JigPlayerSession, EmptyError, ()>(&path, jig::player::instance::Create::METHOD, None).await;

        match status {
            404 => {
                state.error.set(true);
            },
            _ => match result {
                Err(_) => {}
                Ok(res) => {
                    state.error.set_neq(false);
                    // state.play_jig.set(Some((res.jig_id, res.settings)));
                }
            },
        };

    }));
}
