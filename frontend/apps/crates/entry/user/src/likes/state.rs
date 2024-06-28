use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use shared::domain::asset::AssetType;

use super::liked_section::LikedSection;

pub struct Likes {
    pub loader: AsyncLoader,
    pub jigs: Rc<LikedSection>,
    pub resources: Rc<LikedSection>,
    pub playlists: Rc<LikedSection>,
}

impl Likes {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            jigs: LikedSection::new(AssetType::Jig),
            resources: LikedSection::new(AssetType::Resource),
            playlists: LikedSection::new(AssetType::Playlist),
        })
    }
}
