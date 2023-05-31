use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::AssetId,
    circle::Circle,
    jig::JigResponse,
    playlist::PlaylistResponse,
    resource::ResourceResponse,
    user::{public_user::PublicUser, UserId},
};

use crate::state::Community;

pub struct MemberDetails {
    pub member_id: UserId,
    pub member: Mutable<Option<PublicUser>>,
    pub circles: Mutable<Vec<Circle>>,
    pub loader: AsyncLoader,
    pub creations: Mutable<Creations>,
    pub connections: Mutable<Connections>,
    pub play_asset: Mutable<Option<AssetId>>,
    pub community_state: Rc<Community>,
    pub(super) active_popup: Mutable<Option<ActivePopup>>,
}

impl MemberDetails {
    pub fn new(community_state: Rc<Community>, member_id: UserId) -> Rc<Self> {
        Rc::new(Self {
            member_id,
            member: Mutable::new(None),
            circles: Mutable::new(vec![]),
            loader: AsyncLoader::new(),
            creations: Mutable::new(Creations::Jigs(None)),
            connections: Mutable::new(Connections::Followers(None)),
            play_asset: Mutable::new(None),
            community_state,
            active_popup: Mutable::new(None),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Creations {
    Jigs(Option<Vec<JigResponse>>),
    Resources(Option<Vec<ResourceResponse>>),
    Playlists(Option<Vec<PlaylistResponse>>),
}

#[derive(Clone, Debug)]
pub enum Connections {
    Followers(Option<Vec<PublicUser>>),
    Following(Option<Vec<PublicUser>>),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ActivePopup {
    About,
    Bio,
    Image,
}
