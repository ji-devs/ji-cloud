use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::playlist::PlaylistUpdateDraftDataPath};
use utils::prelude::ApiEndpointExt;

use super::state::PlaylistSettings;

impl PlaylistSettings {
    pub fn update_playlist_settings(self: &Rc<Self>) {
        let state = self;
        let req = state.get_playlist_update_req();

        state.loader.load(clone!(state => async move {
            let _ = endpoints::playlist::UpdateDraftData::api_with_auth_empty(
                PlaylistUpdateDraftDataPath(state.playlist.id),
                Some(req),
            )
            .await;
        }));
    }
}
