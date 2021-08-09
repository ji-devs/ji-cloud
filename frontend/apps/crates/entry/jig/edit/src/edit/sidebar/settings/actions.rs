use std::rc::Rc;

use shared::{
    api::{endpoints, ApiEndpoint},
    domain::jig::AudioBackground,
    error::EmptyError,
};
use utils::prelude::api_with_auth_empty;

use super::state::{ActiveSettingsPopup, State};

pub fn on_background_audio_click(state: Rc<State>, audio_background: AudioBackground) {
    let mut selected_audio = state.background_audio.lock_mut();

    *selected_audio = match &*selected_audio {
        Some(selected_audio) if selected_audio == &audio_background => None,
        _ => Some(audio_background),
    };
}

pub fn set_active_popup(state: Rc<State>, active_popup: ActiveSettingsPopup) {
    state.active_popup.set(Some(active_popup));
}

pub fn update_jig_settings(state: Rc<State>) {
    let req = state.get_jig_update_req();

    let path = endpoints::jig::Update::PATH.replace("{id}", &state.jig_id.0.to_string());

    state.loader.load(async move {
        match api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::Update::METHOD, Some(req))
            .await
        {
            Ok(_) => {}
            Err(_) => {}
        };
    });
}
