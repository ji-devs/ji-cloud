use std::rc::Rc;

use super::PlayerPopup;
use dominator::clone;
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::pro_dev::ProDevResponse;

impl PlayerPopup {
    pub fn page_back_signal(&self) -> impl Signal<Item = bool> {
        self.player_state
            .current_page
            .signal()
            .map(move |current_page| {
                let current_page = current_page.unwrap_or(0);
                if current_page > 0 {
                    false
                } else {
                    true
                }
            })
    }

    pub fn page_forward_signal(&self, pro_dev: &Rc<ProDevResponse>) -> impl Signal<Item = bool> {
        self.player_state
            .current_page
            .signal()
            .map(clone!(pro_dev => move |current_page| {
                let current_page = current_page.unwrap_or(0);
                let num_pages = (pro_dev.pro_dev_data.units.len() + 9) / 10;
                if current_page < (num_pages - 1)  {
                    false
                } else {
                    true
                }
            }))
    }
}
