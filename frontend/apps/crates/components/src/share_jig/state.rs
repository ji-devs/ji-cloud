use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;

pub struct State {
    pub active_popup: Mutable<Option<ActivePopup>>,
    pub jig_id: JigId,
}

impl State {
    pub fn new(jig_id: JigId) -> Self {
        Self {
            jig_id,
            active_popup: Mutable::new(None),
        }
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    ShareMain,
    ShareStudents,
    ShareEmbed,
}