use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetId;
use utils::{
    events,
    iframe::{IframeInit, JigPlayerToPlayerPopup},
    prelude::SETTINGS,
    routes::{AssetPlayRoute, AssetRoute, Route},
    unwrap::UnwrapJiExt,
};

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
                            .property("allow", "autoplay; fullscreen")
                            .property("src", {
                                let url = match state.asset_id {
                                    AssetId::JigId(jig_id) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Jig(jig_id, None, state.player_options.clone()))),
                                    AssetId::CourseId(course_id) => Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(course_id))),
                                }.to_string();

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
