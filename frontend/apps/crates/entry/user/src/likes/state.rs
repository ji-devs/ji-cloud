use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::asset::{AssetId, AssetType};

use super::liked_section::LikedSection;

pub struct Likes {
    pub jigs: Rc<LikedSection>,
    pub resources: Rc<LikedSection>,
    pub playlists: Rc<LikedSection>,
    pub play_asset: Mutable<Option<AssetId>>,
}

impl Likes {
    pub fn new() -> Rc<Self> {
        let play_asset = Mutable::new(None);
        Rc::new(Self {
            jigs: LikedSection::new(AssetType::Jig, &play_asset),
            resources: LikedSection::new(AssetType::Resource, &play_asset),
            playlists: LikedSection::new(AssetType::Playlist, &play_asset),
            play_asset,
        })
    }
}
