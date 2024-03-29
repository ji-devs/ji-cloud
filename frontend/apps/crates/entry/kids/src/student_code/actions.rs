use std::rc::Rc;

use dominator::clone;
use futures_signals::signal::Mutable;
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

use super::state::{PlayJig, StudentCode};

impl StudentCode {
    pub fn submit_code(self: &Rc<Self>, number: String) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let res = code_to_jig_id(number).await;

            match res {
                Err(_) => {
                    state.error.set(true)
                },
                Ok(res) => {
                    state.error.set_neq(false);
                    state.play_jig.set(Some(PlayJig {
                        id: res.jig_id,
                        settings: res.settings,
                        token: res.token,
                        name: Mutable::new(None),
                    }));
                },
            }
        }));
    }
}

async fn code_to_jig_id(number: String) -> Result<PlayerSessionInstanceResponse, ()> {
    let number = number.parse::<i32>().map_err(|_| ())?;
    let code = JigCode(number);
    let req = PlayerSessionInstanceCreateRequest { code };

    jig::codes::instance::Create::api_no_auth(PlayerSessionInstanceCreatePath(), Some(req))
        .await
        .map_err(|_| ())
}
