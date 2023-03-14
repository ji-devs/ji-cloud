use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints::{self, pro_dev::unit},
    domain::pro_dev::unit::{
        CreateProDevUnitPath, GetProDevUnitDraftPath, ProDevUnit, ProDevUnitCreateRequest,
        ProDevUnitId as UnitId,
    },
};
use utils::{
    editable_asset::EditableAsset,
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, ProDevEditRoute, Route},
};

use super::UnitEditor;

impl UnitEditor {
    pub async fn get_unit(self: &Rc<Self>, unit_id: UnitId) -> Rc<ProDevUnit> {
        let unit = match &*self.asset_edit_state.asset {
            EditableAsset::ProDev(pro_dev) => pro_dev
                .units
                .lock_ref()
                .iter()
                .find(|unit| unit.id == unit_id)
                .cloned(),
            _ => unreachable!(),
        };

        match unit {
            Some(unit) => Rc::new(unit),
            None => Rc::new(self.load_unit(&unit_id).await),
        }
    }

    async fn load_unit(self: &Rc<Self>, unit_id: &UnitId) -> ProDevUnit {
        let state = self;
        match endpoints::pro_dev::unit::GetDraft::api_with_auth(
            GetProDevUnitDraftPath(
                state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                unit_id.clone(),
            ),
            None,
        )
        .await
        {
            Ok(unit) => unit,
            Err(_) => {
                todo!()
            }
        }
    }

    pub async fn create_async(self: &Rc<Self>) {
        let state = Rc::clone(&self);

        let value = if let Some(value) = self.value.lock_ref().clone() {
            value
        } else {
            todo!();
        };

        let body = ProDevUnitCreateRequest {
            display_name: self.display_name.lock_ref().clone(),
            description: self.description.lock_ref().clone(),
            value,
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
}
