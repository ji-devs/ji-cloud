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

                    // TODO: change to popup
                    let location = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .location()
                        .unwrap();

                    let _ = location.set_href(&Route::Jig(JigRoute::Play(res.jig_id, None, res.settings)).to_string());
                }
            },
        };

    }));
}
