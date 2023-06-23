use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::AssetId,
    circle::Circle,
    course::CourseResponse,
    jig::JigResponse,
    playlist::PlaylistResponse,
    resource::ResourceResponse,
    user::{public_user::PublicUser, UserId},
};

use crate::state::Community;

pub struct MemberDetails {
    pub member_id: UserId,
    pub member: Mutable<Option<PublicUser>>,
    pub is_following: Mutable<Option<bool>>,
    pub loader: AsyncLoader,
    pub jigs: Mutable<Option<Vec<JigResponse>>>,
    pub jigs_count: Mutable<Option<u64>>,
    pub resources: Mutable<Option<Vec<ResourceResponse>>>,
    pub resources_count: Mutable<Option<u64>>,
    pub playlists: Mutable<Option<Vec<PlaylistResponse>>>,
    pub playlists_count: Mutable<Option<u64>>,
    pub courses: Mutable<Option<Vec<CourseResponse>>>,
    pub courses_count: Mutable<Option<u64>>,
    pub circles: Mutable<Option<Vec<Circle>>>,
    pub circles_count: Mutable<Option<u64>>,
    pub followers: Mutable<Option<Vec<PublicUser>>>,
    pub followers_count: Mutable<Option<u64>>,
    pub following: Mutable<Option<Vec<PublicUser>>>,
    pub following_count: Mutable<Option<u64>>,
    pub play_asset: Mutable<Option<AssetId>>,
    pub community_state: Rc<Community>,
    pub(super) active_popup: Mutable<Option<ActivePopup>>,
}

impl MemberDetails {
    pub fn new(community_state: Rc<Community>, member_id: UserId) -> Rc<Self> {
        Rc::new(Self {
            member_id,
            community_state,
            member: Mutable::new(None),
            is_following: Mutable::new(None),
            loader: AsyncLoader::new(),
            jigs: Mutable::new(None),
            resources: Mutable::new(None),
            playlists: Mutable::new(None),
            courses: Mutable::new(None),
            circles: Mutable::new(None),
            followers: Mutable::new(None),
            following: Mutable::new(None),
            play_asset: Mutable::new(None),
            active_popup: Mutable::new(None),
            jigs_count: Mutable::new(None),
            resources_count: Mutable::new(None),
            playlists_count: Mutable::new(None),
            courses_count: Mutable::new(None),
            circles_count: Mutable::new(None),
            followers_count: Mutable::new(None),
            following_count: Mutable::new(None),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ActivePopup {
    About,
    Bio,
    Image,
}
