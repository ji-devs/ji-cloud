use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::{events, prelude::SETTINGS, routes::{JigRoute, Route}, unwrap::UnwrapJiExt, iframe::{JigPlayerToPlayerPopup, IframeInit}};

use super::state::PlayerPopup;

impl PlayerPopup {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        html!("player-popup", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
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
                            .style("border", "0")
                            .property("slot", "iframe")
                            .property("src", {
                                let url: String = Route::Jig(JigRoute::Play(state.jig_id, None, state.player_options.clone())).into();
                                let url = unsafe {
                                    SETTINGS.get_unchecked()
                                        .remote_target
                                        .spa_iframe(&url)
                                };
                                url
                            })
                            .global_event(clone!(state => move |event: events::Message| {
                                if let Ok(data) = event.try_serde_data::<IframeInit<JigPlayerToPlayerPopup>>() {
                                    match data.data {
                                        JigPlayerToPlayerPopup::Close => {
                                            (state.callbacks.close)();
                                        },
                                    }
                                }
                            }))
                        }))
                    },
                }
            })))
        })
    }
}
