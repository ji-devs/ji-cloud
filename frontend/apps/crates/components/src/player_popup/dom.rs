use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::{events, routes::{JigRoute, Route}};

use super::state::PlayerPopup;

impl PlayerPopup {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("player-popup", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap())
            })
            .child(html!("button", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    (state.callbacks.close)();
                }))
            }))
            .child_signal(state.open.signal().map(clone!(state => move |open| {
                match open {
                    false => None,
                    true => {
                        Some(html!("iframe", {
                            .property("slot", "iframe")
                            .property("src", {
                                let url: String = Route::Jig(JigRoute::Play(state.jig_id.clone(), None, state.player_options.clone())).into();
                                url
                            })
                        }))
                    },
                }
            })))
        })
    }
}
