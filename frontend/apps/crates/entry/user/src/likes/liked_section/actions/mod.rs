use std::rc::Rc;

use dominator::clone;
use shared::domain::asset::{AssetId, AssetType};
use wasm_bindgen_futures::spawn_local;

use super::LikedSection;

mod jig_actions;
mod playlist_actions;
mod resource_actions;

impl LikedSection {
    pub(in crate::likes) async fn load_items(self: &Rc<Self>) {
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
    }

    pub(super) fn unlike(self: &Rc<Self>, asset_id: AssetId) {
        let state = self;
        spawn_local(clone!(state => async move {
            let list = match asset_id {
                AssetId::JigId(jig_id) => {
                    state.unlike_jig(jig_id).await
                }
                AssetId::PlaylistId(playlist_id) => {
                    state.unlike_playlist(playlist_id).await
                }
                AssetId::ResourceId(resource_id) => {
                    state.unlike_resource(resource_id).await
                }
                AssetId::CourseId(_) => todo!(),
            };
            list.lock_mut().retain(|asset| asset.id() != asset_id);
        }))
    }
}
