use std::{collections::HashSet, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    jig::{JigId, JigResponse},
    meta::ResourceType,
    playlist::{PlaylistId, PlaylistResponse},
};
use utils::asset::PlaylistPlayerOptions;

pub struct PlaylistPlayer {
    pub playlist_id: PlaylistId,
    pub playlist: Mutable<Option<PlaylistResponse>>,
    pub jigs: Mutable<Vec<JigResponse>>,
    pub jigs_done: Mutable<HashSet<JigId>>,
    pub loader: AsyncLoader,
    pub player_options: PlaylistPlayerOptions,
    pub active_jig: Mutable<Option<JigId>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
}

impl PlaylistPlayer {
    pub fn new(playlist_id: PlaylistId, player_options: PlaylistPlayerOptions) -> Rc<Self> {
        Rc::new(Self {
            playlist_id,
            playlist: Mutable::new(None),
            jigs: Mutable::new(vec![]),
            jigs_done: Default::default(),
            loader: AsyncLoader::new(),
            player_options,
            active_jig: Mutable::new(None),
            resource_types: Default::default(),
        })
    }
}
