use futures_signals::signal::Mutable;
use shared::domain::{asset::AssetId, jig::JigFocus};
use utils::{jig::JigPlayerOptions, routes::JigEditRoute, storage, unwrap::UnwrapJiExt};

pub struct State {
    pub route: Mutable<JigEditRoute>,
    pub jig_focus: JigFocus,
    pub asset_id: AssetId,
    pub show_onboarding: Mutable<bool>,
    pub(super) play_jig: Mutable<Option<AssetPlayerSettings>>,
}

impl State {
    pub fn new(asset_id: AssetId, jig_focus: JigFocus, route: JigEditRoute) -> Self {
        let show_onboarding = storage::get_local_storage()
            .unwrap_ji()
            .get_item("onboarding")
            .unwrap_ji()
            .is_none(); // We don't care about the value, only that the item is present

        Self {
            asset_id,
            jig_focus,
            route: Mutable::new(route),
            play_jig: Mutable::new(None),
            show_onboarding: Mutable::new(show_onboarding),
        }
    }
}


#[derive(Clone)]
pub(super) enum AssetPlayerSettings {
    Jig(JigPlayerOptions),
}
