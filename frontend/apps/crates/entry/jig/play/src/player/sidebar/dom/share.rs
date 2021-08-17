use std::rc::Rc;

use components::share_jig;
use dominator::{html, Dom};
use futures_signals::signal::SignalExt;

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {

    let state = Rc::new(share_jig::state::State::new(state.player_state.jig_id));

    let anchor = html!("jig-play-sidebar-action", {
        .property("kind", "share")
        .property_signal("active", state.active_popup.signal_cloned().map(|active| active.is_some()))
    });

    share_jig::dom::render(state, anchor, Some("actions"))
}
