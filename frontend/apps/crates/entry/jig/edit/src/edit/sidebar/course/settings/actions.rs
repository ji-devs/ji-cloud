use std::rc::Rc;

use shared::{
    api::{endpoints, ApiEndpoint},
    domain::course::AudioBackground,
    error::EmptyError,
};
use utils::prelude::api_with_auth_empty;

use super::state::{ActiveSettingsPopup, State};

pub fn on_background_audio_click(
    state: Rc<State>,
    selected: bool,
    audio_background: AudioBackground,
) {
    if selected {
        state.background_audio.set(Some(audio_background));
    } else {
        state.background_audio.set(None);
    }
    update_course_settings(Rc::clone(&state));
}

pub fn set_active_popup(state: Rc<State>, active_popup: ActiveSettingsPopup) {
    state.active_popup.set(Some(active_popup));
}

pub fn update_course_settings(state: Rc<State>) {
    let req = state.get_course_update_req();

    let path = endpoints::course::UpdateDraftData::PATH.replace("{id}", &state.course_id.0.to_string());

    state.loader.load(async move {
        let _ = api_with_auth_empty::<EmptyError, _>(
            &path,
            endpoints::course::UpdateDraftData::METHOD,
            Some(req),
        )
        .await;
    });
}
