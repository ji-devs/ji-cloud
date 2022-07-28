use std::rc::Rc;

use shared::domain::asset::AssetType;

use super::SearchResultsSection;

mod course_actions;
mod jig_actions;
mod resource_actions;

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
                self.load_jigs().await;
            }
        }
    }
}
