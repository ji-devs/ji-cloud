use std::rc::Rc;

use super::sidebar::SidebarSpot;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::asset::AssetId;
use utils::{
    asset::AssetPlayerOptions,
    editable_asset::EditableAsset,
    routes::{AssetEditRoute, CourseEditRoute, JigEditRoute, PlaylistEditRoute, ResourceEditRoute},
    storage,
    unwrap::UnwrapJiExt,
};

pub struct AssetEditState {
    pub asset_id: AssetId,
    pub asset: Rc<EditableAsset>,
    // using this instead of jig.modules/playlist.items
    pub sidebar_spots: MutableVec<Rc<SidebarSpot>>,
    pub route: Mutable<AssetEditRoute>,
    pub show_onboarding: Mutable<bool>,
    pub target_index: Mutable<Option<usize>>,
    pub(super) play_jig: Mutable<Option<AssetPlayerOptions>>,
}

impl AssetEditState {
    pub fn new(asset_id: AssetId, route: AssetEditRoute) -> Rc<Self> {
        let show_onboarding = storage::get_local_storage()
            .unwrap_ji()
            .get_item("onboarding")
            .unwrap_ji()
            .is_none(); // We don't care about the value, only that the item is present

        Rc::new(Self {
            asset_id,
            asset: Rc::new(asset_id.into()),
            sidebar_spots: Default::default(),
            route: Mutable::new(route),
            play_jig: Mutable::new(None),
            target_index: Mutable::new(None),
            show_onboarding: Mutable::new(show_onboarding),
        })
    }

    pub fn set_route_jig(&self, route: JigEditRoute) {
        assert!(&self.asset_id.is_jig_id());
        self.route
            .set(AssetEditRoute::Jig(*self.asset_id.unwrap_jig(), route));
    }

    pub fn set_route_resource(&self, route: ResourceEditRoute) {
        assert!(&self.asset_id.is_resource_id());
        self.route.set(AssetEditRoute::Resource(
            *self.asset_id.unwrap_resource(),
            route,
        ));
    }

    pub fn set_route_playlist(&self, route: PlaylistEditRoute) {
        assert!(&self.asset_id.is_playlist_id());
        self.route.set(AssetEditRoute::Playlist(
            *self.asset_id.unwrap_playlist(),
            route,
        ));
    }
    pub fn set_route_course(&self, route: CourseEditRoute) {
        assert!(&self.asset_id.is_course_id());
        self.route.set(AssetEditRoute::Course(
            *self.asset_id.unwrap_course(),
            route,
        ));
    }
}
