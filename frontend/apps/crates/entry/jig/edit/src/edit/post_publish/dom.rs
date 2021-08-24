use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::{JigId, JigPlayerSettings};
use utils::{events, routes::{JigRoute, Route}};

use super::{
    actions,
    state::*,
    super::state::State as JigEditState,
};
use components::share_jig;
use std::rc::Rc;


pub fn render(jig_id: JigId, jig_edit_state: Rc<JigEditState>) -> Dom {
    let state = Rc::new(State::new(jig_id, jig_edit_state));

    let share_anchor = html!("post-publish-action", {
        .property("kind", "share")
        .property_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
    });

    html!("post-publish", {
        .property("slot", "main")
        .children(&mut [
            share_jig::dom::render(Rc::clone(&state.share_state), share_anchor, Some("actions")),
            html!("post-publish-action", {
                .property("slot", "actions")
                .property("kind", "new-jig")
                .event(clone!(state => move |_: events::Click| {
                    actions::create_jig(Rc::clone(&state));
                }))
            }),
            html!("post-publish-action", {
                .property("kind", "play-jig")
                .property("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    state.jig_edit_state.play_jig.set(Some(JigPlayerSettings::default()));
                }))
            }),
        ])
    })
}
