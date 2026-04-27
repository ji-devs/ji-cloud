use std::rc::Rc;

use super::state::PlayerPopup;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetId;
use utils::{
    events,
    iframe::{AssetPlayerToPlayerPopup, IframeInit},
    prelude::SETTINGS,
    unwrap::UnwrapJiExt,
};

impl PlayerPopup {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        if state.signed_url.lock_ref().is_none() {
            state.load_signed_url();
        }
        html!("player-popup", {
            .prop("size", match state.asset_id {
                AssetId::JigId(_) => "aspect-ratio",
                AssetId::PlaylistId(_) => "full-screen",
                AssetId::ResourceId(_) => unreachable!(),
                AssetId::CourseId(_) => "full-screen",
            })
            .prop("preview", state.player_options.is_draft())
            .apply_if(slot.is_some(), |dom| {
                dom.prop("slot", slot.unwrap_ji())
            })
            .child_signal(state.close_button_shown.signal().map(clone!(state => move |close_button_shown| {
                match close_button_shown {
                    false => None,
                    true => {
                        Some(html!("fa-button", {
                            .prop("slot", "close")
                            .prop("icon", "fa-light fa-xmark")
                            .event(clone!(state => move |_: events::Click| {
                                (state.callbacks.close)();
                            }))
                        }))
                    },
                }
            })))
            .child_signal(state.signed_url.signal_cloned().map(clone!(state => move |signed_url| {
                match (state.open.get(), signed_url) {
                    (true, Some(signed_url)) => {
                        Some(html!("iframe", {
                            .style("border", "0")
                            .prop("slot", "iframe")
                            .prop("allow", "autoplay; fullscreen")
                            .prop("src", iframe_signed_url(&signed_url))
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
                    _ => None,
                }
            })))
        })
    }
}

fn iframe_signed_url(signed_url: &str) -> String {
    let remote_target = &SETTINGS.get().unwrap_ji().remote_target;
    let pages_url = remote_target.pages_url();

    match signed_url.strip_prefix(&pages_url) {
        Some(route_path) => remote_target.spa_iframe(route_path),
        None => signed_url.to_string(),
    }
}
