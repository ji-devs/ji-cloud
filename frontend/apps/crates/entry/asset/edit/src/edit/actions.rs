use super::state::AssetEditState;
use shared::domain::asset::AssetId;
use utils::{
    prelude::ModuleToAssetEditorMessage,
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, ResourceEditRoute, Route},
    storage,
    unwrap::UnwrapJiExt,
};

impl AssetEditState {
    pub fn set_permanently_closed(&self) {
        let _ = storage::get_local_storage()
            .unwrap_ji()
            .set_item("onboarding", "hidden");
        self.show_onboarding.set_neq(false);
    }

    pub fn on_iframe_message(&self, message: ModuleToAssetEditorMessage) {
        match message {
            ModuleToAssetEditorMessage::Publish => {
                self.navigate_to_publish();
            }
        }
    }

    pub fn navigate_to_publish(&self) {
        match self.asset_id {
            AssetId::JigId(jig_id) => {
                self.set_route_jig(JigEditRoute::Publish);
                Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                    jig_id,
                    JigEditRoute::Publish,
                ))));
            }
            AssetId::CourseId(course_id) => {
                self.set_route_course(CourseEditRoute::Publish);
                Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                    course_id,
                    CourseEditRoute::Publish,
                ))));
            }
            AssetId::ResourceId(resource_id) => {
                self.set_route_resource(ResourceEditRoute::Landing);
                Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                    resource_id,
                    ResourceEditRoute::Landing,
                ))));
            }
        }
    }
}
