use super::state::PlayerPopup;
use crate::audio::mixer::audio_iframe_messenger;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetId;
use std::rc::Rc;
use utils::{
    asset::AssetPlayerOptions,
    events,
    iframe::{AssetPlayerToPlayerPopup, IframeInit},
    prelude::SETTINGS,
    routes::{AssetPlayRoute, AssetRoute, Route},
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlIFrameElement;

impl PlayerPopup {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        html!("player-popup", {
            .property("size", match state.asset_id {
                AssetId::JigId(_) => "aspect-ratio",
                AssetId::CourseId(_) => "full-screen",
            })
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .child_signal(state.close_button_shown.signal().map(clone!(state => move |close_button_shown| {
                match close_button_shown {
                    false => None,
                    true => {
                        Some(html!("button", {
                            .property("slot", "close")
                            .text("×")
                            .event(clone!(state => move |_: events::Click| {
                                (state.callbacks.close)();
                            }))
                        }))
                    },
                }
            })))
            .child_signal(state.open.signal().map(clone!(state => move |open| {
                match open {
                    false => None,
                    true => {
                        Some(html!("iframe" => HtmlIFrameElement, {
                            .style("border", "0")
                            .apply(audio_iframe_messenger)
                            .property("slot", "iframe")
                            .property("allow", "autoplay; fullscreen")
                            .property("src", {
                                let url = match (state.asset_id, &state.player_options) {
                                    (AssetId::JigId(jig_id), AssetPlayerOptions::Jig(player_options)) => {
                                        Route::Asset(AssetRoute::Play(AssetPlayRoute::Jig(jig_id, None, player_options.clone())))
                                    },
                                    (AssetId::CourseId(course_id), AssetPlayerOptions::Course(player_options)) => {
                                        Route::Asset(AssetRoute::Play(AssetPlayRoute::Course(course_id, player_options.clone())))
                                    },
                                    _ => {
                                        panic!("Invalid asset id/player_options combinations")
                                    }
                                }.to_string();

                                let url = unsafe {
                                    SETTINGS.get_unchecked()
                                        .remote_target
                                        .spa_iframe(&url)
                                };
                                url
                            })
                            .global_event(clone!(state => move |event: events::Message| {
                                if let Ok(data) = event.try_serde_data::<IframeInit<AssetPlayerToPlayerPopup>>() {
                                    match data.data {
                                        AssetPlayerToPlayerPopup::Close => {
                                            (state.callbacks.close)();
                                        },
                                        AssetPlayerToPlayerPopup::CloseButtonShown(shown) => {
                                            state.close_button_shown.set_neq(shown);
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
