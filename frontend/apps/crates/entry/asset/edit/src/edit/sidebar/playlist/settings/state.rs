use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::playlist::PlaylistUpdateDraftDataRequest;
use utils::editable_asset::EditablePlaylist;

pub struct PlaylistSettings {
    pub playlist: Rc<EditablePlaylist>,
    pub popup_open: Mutable<bool>,
    pub play_in_order: Mutable<bool>,
    pub loader: AsyncLoader,
}

impl PlaylistSettings {
    pub fn new(playlist: &Rc<EditablePlaylist>) -> Rc<Self> {
        Rc::new(Self {
            playlist: Rc::clone(playlist),
            popup_open: Mutable::new(false),
            play_in_order: Mutable::new(false),
            loader: AsyncLoader::new(),
        })
    }

    pub fn get_playlist_update_req(&self) -> PlaylistUpdateDraftDataRequest {
        PlaylistUpdateDraftDataRequest {
            ..Default::default()
        }
    }

    // fn get_player_settings(&self) -> PlaylistPlayerSettings {
    //     PlaylistPlayerSettings {
    //         direction: self.direction.get(),
    //         display_score: self.display_score.get(),
    //         track_assessments: self.track_assessments.get(),
    //         drag_assist: self.drag_assist.get(),
    //     }
    // }
}
