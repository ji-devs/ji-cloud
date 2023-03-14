use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints::{self, pro_dev::unit},
    domain::pro_dev::unit::{
        CreateProDevUnitPath, GetProDevUnitDraftPath, ProDevUnit, ProDevUnitCreateRequest,
        ProDevUnitId as UnitId, ProDevUnitUpdateRequest, ProDevUnitValue, UpdateProDevUnitPath,
    },
};
use utils::{
    editable_asset::EditableAsset,
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, ProDevEditRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::UnitEditor;

impl UnitEditor {
    pub fn load_unit(self: &Rc<Self>) {
        if let Some(unit_id) = self.unit_id {
            let units = self.editable_pro_dev.units.lock_ref();
            let unit = units.iter().find(|x| x.id == unit_id);

            match unit {
                Some(unit) => {
                    self.display_name.set(unit.display_name.clone());
                    self.description.set(unit.description.clone());
                    self.value.set(unit.value.clone().into());
                }
                None => {}
            }
        };
    }

    pub async fn create_async(self: &Rc<Self>) {
        let state = Rc::clone(&self);

        let body = ProDevUnitCreateRequest {
            display_name: self.display_name.lock_ref().clone(),
            description: self.description.lock_ref().clone(),
            value: ProDevUnitValue::try_from(self.value.lock_ref().clone()).unwrap_ji(),
        };

        let _ = unit::Create::api_with_auth_empty(
            CreateProDevUnitPath(state.asset_edit_state.asset_id.unwrap_pro_dev().clone()),
            Some(body),
        )
        .await;
    }

    pub fn create_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.create_async().await;
            let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
                state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                ProDevEditRoute::Unit(state.unit_id),
            )))
            .into();

            dominator::routing::go_to_url(&url);
        }));
    }

    pub async fn update_async(self: &Rc<Self>) {
        let state = Rc::clone(&self);

        let body = ProDevUnitUpdateRequest {
            display_name: Some(self.display_name.lock_ref().clone()),
            description: Some(self.description.lock_ref().clone()),
            value: Some(ProDevUnitValue::try_from(self.value.lock_ref().clone()).unwrap_ji()),
            index: None,
        };

        if let Some(unit_id) = self.unit_id {
            let _ = unit::Update::api_with_auth_empty(
                UpdateProDevUnitPath(
                    state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                    unit_id,
                ),
                Some(body),
            )
            .await;
        };
    }

    pub fn update_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.update_async().await;
        }));

        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
            state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
            ProDevEditRoute::Unit(state.unit_id),
        )))
        .into();

        dominator::routing::go_to_url(&url);
    }
}
