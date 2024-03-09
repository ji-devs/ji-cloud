use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::playlist::PlaylistId;
use std::rc::Rc;
use utils::editable_asset::EditablePlaylist;

use crate::playlist_curation::PlaylistCuration;

pub struct PlaylistDetails {
    pub playlist_id: PlaylistId,
    pub playlist: Rc<EditablePlaylist>,
    pub loader: AsyncLoader,
    pub curation_state: Rc<PlaylistCuration>,
    pub player_open: Mutable<bool>,
}

impl PlaylistDetails {
    pub fn new(
        curation_state: Rc<PlaylistCuration>,
        playlist_id: PlaylistId,
        playlist: Rc<EditablePlaylist>,
    ) -> Rc<Self> {
        Rc::new(Self {
            playlist_id,
            playlist,
            loader: AsyncLoader::new(),
            curation_state,
            player_open: Mutable::new(false),
        })
    }
}
