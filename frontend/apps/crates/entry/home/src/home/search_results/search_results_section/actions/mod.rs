use std::rc::Rc;

use futures_signals::signal::Mutable;
use gloo::utils::document;
use serde::{Deserialize, Serialize};
use shared::{
    api::endpoints,
    domain::{
        asset::{AssetId, AssetType},
        jig::{JigLikePath, JigUnlikePath},
        playlist::{PlaylistLikePath, PlaylistUnlikePath},
        resource::{ResourceLikePath, ResourceUnlikePath},
    },
};
use utils::{
    init::user::get_plan_tier,
    js_object, paywall,
    prelude::{is_user_set, ApiEndpointExt},
    storage,
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

use super::SearchResultsSection;

mod jig_actions;
mod playlist_actions;
mod resource_actions;

const PLAYED_WITHOUT_LOGIN_COUNT_KEY: &'static str = "PLAYED_WITHOUT_LOGIN_COUNT";
const PLAYED_WITHOUT_TODAY_FREE_KEY: &'static str = "PLAYED_WITHOUT_TODAY_FREE";
const PLAYED_WITHOUT_LOGIN_ALLOWED: u32 = 5;
const PLAYED_FREE_ACCOUNT_ALLOWED_DAILY: u32 = 5;

fn get_page_pos() -> i32 {
    document().document_element().unwrap_ji().scroll_top()
}
fn set_page_pos(pos: i32) {
    document()
        .document_element()
        .unwrap_ji()
        .set_scroll_top(pos)
}

impl SearchResultsSection {
    pub async fn load_items(self: &Rc<Self>) {
        let pos = get_page_pos();
        match self.asset_type {
            AssetType::Jig => {
                self.load_jigs().await;
            }
            AssetType::Resource => {
                self.load_resources().await;
            }
            AssetType::Playlist => {
                self.load_playlists().await;
            }
            AssetType::Course => todo!(),
        }
        utils::request_animation_frame::before_next_frame().await;
        set_page_pos(pos);
    }

    pub fn on_play_asset_click(self: &Rc<Self>, asset_id: AssetId) {
        match play_blocked() {
            Some(Block::FreeAccountLimit) => {
                paywall::dialog_limit(&format!(
                    "
                    Looking to play more than {} JIGs in one day?
                    Upgrade now for UNLIMITED daily JIG plays.
                ",
                    PLAYED_FREE_ACCOUNT_ALLOWED_DAILY
                ));
            }
            Some(Block::NoAccountLimit) => {
                self.home_state.play_login_popup_shown.set(true);
            }
            None => {
                self.home_state.play_asset.set(Some(asset_id));
                increase_played_count();
            }
        }
    }

    pub fn on_like_click(self: &Rc<Self>, asset_id: AssetId, liked_mutable: &Mutable<bool>) {
        let is_liked = !liked_mutable.get();
        liked_mutable.set(is_liked);
        spawn_local(async move {
            match asset_id {
                AssetId::JigId(jig_id) => {
                    match is_liked {
                        true => {
                            endpoints::jig::Like::api_with_auth(JigLikePath(jig_id), None)
                                .await
                                .unwrap_ji();
                        }
                        false => {
                            endpoints::jig::Unlike::api_with_auth(JigUnlikePath(jig_id), None)
                                .await
                                .unwrap_ji();
                        }
                    };
                }
                AssetId::PlaylistId(playlist_id) => {
                    match is_liked {
                        true => {
                            endpoints::playlist::Like::api_with_auth(
                                PlaylistLikePath(playlist_id),
                                None,
                            )
                            .await
                            .unwrap_ji();
                        }
                        false => {
                            endpoints::playlist::Unlike::api_with_auth(
                                PlaylistUnlikePath(playlist_id),
                                None,
                            )
                            .await
                            .unwrap_ji();
                        }
                    };
                }
                AssetId::ResourceId(resource_id) => {
                    match is_liked {
                        true => {
                            endpoints::resource::Like::api_with_auth(
                                ResourceLikePath(resource_id),
                                None,
                            )
                            .await
                            .unwrap_ji();
                        }
                        false => {
                            endpoints::resource::Unlike::api_with_auth(
                                ResourceUnlikePath(resource_id),
                                None,
                            )
                            .await
                            .unwrap_ji();
                        }
                    };
                }
                AssetId::CourseId(_) => todo!(),
            }
        });
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Block {
    NoAccountLimit,
    FreeAccountLimit,
}

fn play_blocked() -> Option<Block> {
    match is_user_set() {
        true => (get_plan_tier().is_free()
            && get_free_played_today_count() >= PLAYED_FREE_ACCOUNT_ALLOWED_DAILY)
            .then(|| Block::FreeAccountLimit),
        false => (get_played_without_login_count() >= PLAYED_WITHOUT_LOGIN_ALLOWED)
            .then(|| Block::NoAccountLimit),
    }
}

fn increase_played_count() {
    match is_user_set() {
        true => {
            if get_plan_tier().is_free() {
                let count = get_free_played_today_count();
                set_free_played_today_count(count + 1);
            }
        }
        false => {
            let count = get_played_without_login_count();
            set_played_without_login_count(count + 1);
        }
    }
}

fn get_played_without_login_count() -> u32 {
    storage::get_local_storage()
        .unwrap_ji()
        .get_item(PLAYED_WITHOUT_LOGIN_COUNT_KEY)
        .unwrap_ji()
        .map(|v| v.parse().unwrap_ji())
        .unwrap_or(0)
}

fn set_played_without_login_count(count: u32) {
    let _ = storage::get_local_storage()
        .unwrap_ji()
        .set_item(PLAYED_WITHOUT_LOGIN_COUNT_KEY, &count.to_string());
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayedDay {
    day: String,
    count: u32,
}

fn get_free_played_today_count() -> u32 {
    storage::get_local_storage()
        .unwrap_ji()
        .get_item(PLAYED_WITHOUT_TODAY_FREE_KEY)
        .unwrap_ji()
        .map(|v| {
            let v: PlayedDay = serde_json::from_str(&v).unwrap();
            if v.day == get_todays_date() {
                v.count
            } else {
                0
            }
        })
        .unwrap_or(0)
}

fn set_free_played_today_count(count: u32) {
    let val = serde_json::to_string(&PlayedDay {
        day: get_todays_date(),
        count,
    })
    .unwrap();
    let _ = storage::get_local_storage()
        .unwrap_ji()
        .set_item(PLAYED_WITHOUT_TODAY_FREE_KEY, &val);
}

fn get_todays_date() -> String {
    let date: js_sys::Date = js_sys::Date::new_0();
    date.to_locale_date_string(
        "sv-SE",
        &js_object!({
            "year": "numeric",
            "month": "2-digit",
            "day": "2-digit",
        }),
    )
    .into()
}
