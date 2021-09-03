use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use utils::jig::JigPlayerOptions;

use super::PreviewPopupCallbacks;


pub struct PlayerPopup {
    pub jig_id: JigId,
    pub player_options: JigPlayerOptions,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
}

impl PlayerPopup {
    pub fn new(jig_id: JigId, player_options: JigPlayerOptions, callbacks: PreviewPopupCallbacks) -> Self {
        Self {
            jig_id,
            player_options,
            open: Mutable::new(true),
            callbacks,
        }
    }
}
