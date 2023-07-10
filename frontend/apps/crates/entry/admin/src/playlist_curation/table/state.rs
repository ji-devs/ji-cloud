use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::playlist_curation::PlaylistCuration;

pub struct PlaylistTable {
    pub loader: AsyncLoader,
    pub curation_state: Rc<PlaylistCuration>,
}

impl PlaylistTable {
    pub fn new(curation_state: Rc<PlaylistCuration>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
