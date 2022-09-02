use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::jig::{AudioBackground, JigUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

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
    update_jig_settings(Rc::clone(&state));
}

pub fn set_active_popup(state: Rc<State>, active_popup: ActiveSettingsPopup) {
    state.active_popup.set(Some(active_popup));
}

pub fn update_jig_settings(state: Rc<State>) {
    let req = state.get_jig_update_req();

    state.loader.load(clone!(state => async move {
        let _ = endpoints::jig::UpdateDraftData::api_with_auth_empty(
            JigUpdateDraftDataPath(state.jig_id.clone()),
            Some(req),
        )
        .await;
    }));
}
