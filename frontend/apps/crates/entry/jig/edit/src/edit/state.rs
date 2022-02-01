use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigFocus};
use utils::{
    jig::JigPlayerOptions,
    routes::JigEditRoute,
    storage,
    unwrap::UnwrapJiExt
};

pub struct State {
    pub route: Mutable<JigEditRoute>,
    pub jig_focus: JigFocus,
    pub jig_id: JigId,
    pub play_jig: Mutable<Option<JigPlayerOptions>>,
    pub show_onboarding: Mutable<bool>,
}

impl State {
    pub fn new(jig_id: JigId, jig_focus: JigFocus, route: JigEditRoute) -> Self {
        let show_onboarding = storage::get_local_storage()
            .unwrap_ji()
            .get_item("onboarding")
            .unwrap_ji()
            .is_none(); // We don't care about the value, only that the item is present

        Self {
            jig_id,
            jig_focus,
            route: Mutable::new(route),
            play_jig: Mutable::new(None),
            show_onboarding: Mutable::new(show_onboarding),
        }
    }
}
