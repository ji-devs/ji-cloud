use std::rc::Rc;

use components::share_asset;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::JigResponse;
use utils::events;

use super::super::state::Sidebar;

impl Sidebar {
    pub fn render_share(self: &Rc<Self>, jig: JigResponse) -> Dom {
        let state = self;
        let share_jig = share_asset::ShareAsset::new(jig.into());

        let anchor = html!("jig-play-sidebar-action", {
            .prop("kind", "share")
            .prop_signal("active", share_jig.active_popup.signal_cloned().map(|active| active.is_some()))
            .event(clone!(state => move |_: events::Click| {
                state.track_action("Share Click");
            }))
        });

        share_jig.render(anchor, Some("actions"))
    }
}
