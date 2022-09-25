use std::rc::Rc;

use components::share_asset;
use dominator::{html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use super::{super::state::State, track_action};

pub fn render(state: Rc<State>) -> Dom {
    let share_jig = share_asset::ShareAsset::new(state.player_state.jig_id.into());

    let anchor = html!("jig-play-sidebar-action", {
        .property("kind", "share")
        .property_signal("active", share_jig.active_popup.signal_cloned().map(|active| active.is_some()))
        .event(move |_: events::Click| {
            track_action("Information Click", state.clone());
        })
    });

    share_jig.render(anchor, Some("actions"))
}
