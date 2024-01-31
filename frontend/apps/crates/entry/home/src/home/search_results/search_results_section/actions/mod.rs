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
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use wasm_bindgen_futures::spawn_local;

use super::SearchResultsSection;

mod jig_actions;
mod playlist_actions;
mod resource_actions;

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
