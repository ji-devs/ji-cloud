use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::jig::{AudioBackground, JigUpdateDraftDataPath},
};
use utils::prelude::ApiEndpointExt;

use super::state::{ActiveSettingsPopup, JigSettings};

pub fn on_background_audio_click(
    state: Rc<JigSettings>,
    selected: bool,
    audio_background: AudioBackground,
) {
    if selected {
        state.jig.audio_background.set(Some(audio_background));
    } else {
        state.jig.audio_background.set(None);
    }
    update_jig_settings(Rc::clone(&state));
}

pub fn set_active_popup(state: Rc<JigSettings>, active_popup: ActiveSettingsPopup) {
    state.active_popup.set(Some(active_popup));
}

pub fn update_jig_settings(state: Rc<JigSettings>) {
    let req = state.get_jig_update_req();

    state.loader.load(clone!(state => async move {
        let _ = endpoints::jig::UpdateDraftData::api_with_auth(
            JigUpdateDraftDataPath(state.jig.id.clone()),
            Some(req),
        )
        .await;
    }));
}
