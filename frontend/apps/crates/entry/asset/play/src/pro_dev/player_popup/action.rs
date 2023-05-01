use std::rc::Rc;

use super::PlayerPopup;
use dominator::clone;
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::pro_dev::ProDevResponse;

impl PlayerPopup {
    pub fn navigate_previous_signal(&self) -> impl Signal<Item = bool> {
        self.player_state.active_unit.signal().map(
            move |active_unit| {
                if active_unit > Some(0) {
                    false
                } else {
                    true
                }
            },
        )
    }

    pub fn navigate_forward_signal(
        &self,
        pro_dev: &Rc<ProDevResponse>,
    ) -> impl Signal<Item = bool> {
        self.player_state
            .active_unit
            .signal()
            .map(clone!(pro_dev => move |active_unit| {
                let last_index = pro_dev.pro_dev_data.units.len();
                if active_unit < Some(last_index - 1)  {
                    false
                } else {
                    true
                }
            }))
    }
}
