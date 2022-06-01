use std::rc::Rc;

use shared::{
    api::{endpoints, ApiEndpoint},
    error::EmptyError,
};
use utils::prelude::api_with_auth_empty;

use super::state::State;

pub fn update_course_settings(state: Rc<State>) {
    let req = state.get_course_update_req();

    let path =
        endpoints::course::UpdateDraftData::PATH.replace("{id}", &state.course_id.0.to_string());

    state.loader.load(async move {
        let _ = api_with_auth_empty::<EmptyError, _>(
            &path,
            endpoints::course::UpdateDraftData::METHOD,
            Some(req),
        )
        .await;
    });
}
