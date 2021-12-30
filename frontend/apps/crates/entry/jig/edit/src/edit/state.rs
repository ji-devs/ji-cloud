use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigFocus};
use utils::{jig::JigPlayerOptions, routes::JigEditRoute};

pub struct State {
    pub route: Mutable<JigEditRoute>,
    pub jig_focus: JigFocus,
    pub jig_id: JigId,
    pub play_jig: Mutable<Option<JigPlayerOptions>>,
}

impl State {
    pub fn new(jig_id: JigId, jig_focus: JigFocus, route: JigEditRoute) -> Self {
        Self {
            jig_id,
            jig_focus,
            route: Mutable::new(route),
            play_jig: Mutable::new(None),
        }
    }
}
