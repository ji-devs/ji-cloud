use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::{AssetId, AssetType};

use super::liked_section::LikedSection;

pub struct Likes {
    pub loader: AsyncLoader,
    pub jigs: Rc<LikedSection>,
    pub resources: Rc<LikedSection>,
    pub playlists: Rc<LikedSection>,
    pub play_asset: Mutable<Option<AssetId>>,
}

impl Likes {
    pub fn new() -> Rc<Self> {
        let play_asset = Mutable::new(None);
        Rc::new(Self {
            loader: AsyncLoader::new(),
            jigs: LikedSection::new(AssetType::Jig, &play_asset),
            resources: LikedSection::new(AssetType::Resource, &play_asset),
            playlists: LikedSection::new(AssetType::Playlist, &play_asset),
            play_asset,
        })
    }
}
