use std::rc::Rc;

use futures_signals::signal::Mutable;
use gloo::utils::document;
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
const PLAYED_WITHOUT_LOGIN_ALLOWED: u32 = 5;

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
        if is_user_set() || under_limit() {
            self.home_state.play_asset.set(Some(asset_id));
            increased_played_count();
        } else {
            self.home_state.play_login_popup_shown.set(true);
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

fn under_limit() -> bool {
    get_played_without_login_count() < PLAYED_WITHOUT_LOGIN_ALLOWED
}

fn increased_played_count() {
    if !is_user_set() {
        let count = get_played_without_login_count();
        set_played_without_login_count(count + 1);
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
