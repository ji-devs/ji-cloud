use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints::jig,
    domain::jig::codes::{
        instance::{
            PlayerSessionInstanceCreatePath, PlayerSessionInstanceCreateRequest,
            PlayerSessionInstanceResponse,
        },
        JigCode,
    },
};
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
    let number = number.parse::<i32>().map_err(|_| ())?;
    let code = JigCode(number);
    let req = PlayerSessionInstanceCreateRequest { code };

    let (result, status) = jig::codes::instance::Create::api_no_auth_status(
        PlayerSessionInstanceCreatePath(),
        Some(req),
    )
    .await;

    match status {
        404 => Err(()),
        _ => result.map_err(|_| ()),
    }
}
