use std::rc::Rc;

use shared::domain::asset::{AssetId, AssetType};
use utils::{prelude::is_user_set, storage, unwrap::UnwrapJiExt};

use super::SearchResultsSection;

mod course_actions;
mod jig_actions;
mod resource_actions;

const PLAYED_WITHOUT_LOGIN_COUNT_KEY: &'static str = "PLAYED_WITHOUT_LOGIN_COUNT";
const PLAYED_WITHOUT_LOGIN_ALLOWED: u32 = 5;

impl SearchResultsSection {
    pub async fn load_items(self: &Rc<Self>) {
        match self.asset_type {
            AssetType::Jig => {
                self.load_jigs().await;
            }
            AssetType::Resource => {
                self.load_resources().await;
            }
            AssetType::Course => {
                self.load_courses().await;
            }
            AssetType::ProDev => todo!(),
        }
    }

    pub fn on_play_asset_click(self: &Rc<Self>, asset_id: AssetId) {
        if is_user_set() || under_limit() {
            self.home_state.play_asset.set(Some(asset_id));
            increased_played_count();
        } else {
            self.home_state.play_login_popup_shown.set(true);
        }
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
