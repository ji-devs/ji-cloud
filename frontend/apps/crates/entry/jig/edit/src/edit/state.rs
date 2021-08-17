use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use utils::routes::JigEditRoute;

pub struct State {
    pub route: Mutable<JigEditRoute>,
    pub jig_id: JigId,
}

impl State {
    pub fn new(jig_id: JigId, route: JigEditRoute) -> Self {
        Self {
            jig_id,
            route: Mutable::new(route),
        }
    }
}
