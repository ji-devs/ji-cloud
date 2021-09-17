use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints::jig, domain::jig::player::{JigPlayerSessionIndex, instance::{PlayerSessionInstanceCreateRequest, PlayerSessionInstanceResponse}}};
use utils::prelude::*;

use super::state::State;

pub fn submit_code(state: Rc<State>, number: String) {
    state.loader.load(clone!(state => async move {
        match code_to_jig_id(number).await {
            Err(_) => {}
            Ok(res) => {
                state.error.set_neq(false);
                state.play_jig.set(Some((res.jig_id, res.settings)));
            }
        };

    }));
}

async fn code_to_jig_id(number: String) -> Result<PlayerSessionInstanceResponse, ()> {
    let number = number.parse::<i16>().map_err(|_| ())?;
    let index = JigPlayerSessionIndex(number);
    let req = PlayerSessionInstanceCreateRequest {
        index
    };

    let (result, status) = jig::player::instance::Create::api_no_auth_status(Some(req)).await;

    match status {
        404 => Err(()),
        _ => result.map_err(|_| ()),
    }
}
