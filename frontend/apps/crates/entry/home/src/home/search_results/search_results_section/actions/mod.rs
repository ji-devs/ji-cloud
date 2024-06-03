use std::rc::Rc;

use gloo::utils::document;
use shared::domain::asset::AssetType;
use utils::unwrap::UnwrapJiExt;

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
}
