use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints::pro_dev::unit,
    domain::pro_dev::unit::{
        CreateProDevUnitPath, ProDevUnitCreateRequest, ProDevUnitUpdateRequest, ProDevUnitValue,
        UpdateProDevUnitPath,
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, ProDevEditRoute},
    unwrap::UnwrapJiExt,
};

use super::UnitEditor;

const STR_ADD_TO_COURSE: &str = "Add to course";
const STR_UPDATE: &str = " Update unit ";

impl UnitEditor {
    pub fn load_unit(&self) {
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

        let _ = unit::Create::api_with_auth(
            CreateProDevUnitPath(state.asset_edit_state.asset_id.unwrap_pro_dev().clone()),
            Some(body),
        )
        .await;
    }

    pub fn create_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.create_async().await;
            state.asset_edit_state.route.set(AssetEditRoute::ProDev(
                state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                ProDevEditRoute::Unit(None),
            ));
        }));
    }

    pub async fn update_async(&self) -> Result<(), String> {
        let state = self;

        let body = ProDevUnitUpdateRequest {
            display_name: Some(self.display_name.lock_ref().clone()),
            description: Some(self.description.lock_ref().clone()),
            value: Some(ProDevUnitValue::try_from(self.value.lock_ref().clone()).unwrap_ji()),
            index: None,
        };

        if let Some(unit_id) = self.unit_id {
            let res = unit::Update::api_with_auth_empty(
                UpdateProDevUnitPath(
                    state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                    unit_id,
                ),
                Some(body),
            )
            .await;

            match res {
                Ok(_) => {
                    log::info!("update_async(): update successful");
                    Ok(())
                }
                Err(e) => {
                    log::error!("update_async(): update failed: {}", e);
                    Err(format!("update failed: {}", e))
                }
            }
        } else {
            Err(String::from("update_async(): unit ID is None"))
        }
    }

    pub fn update_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            if let Err(msg) = state.update_async().await {
                log::error!("{}", msg);
            } else {
                // deactivate unit submit button request has completed
                state.changed.set(false);

            }

            state.asset_edit_state.route.set(AssetEditRoute::ProDev(
                state.asset_edit_state.asset_id.unwrap_pro_dev().clone(),
                ProDevEditRoute::Unit(state.unit_id),
            ));
        }));

        log::info!("Success");
    }

    pub fn on_display_name_change(self: &Rc<Self>, new_name: &str) {
        self.changed.set(true);
        self.display_name.set(new_name.to_string());

        log::info!("display_name {:?}", self.display_name);
    }

    pub fn on_description_change(self: &Rc<Self>, new_description: &str) {
        self.changed.set(true);
        self.description.set(new_description.to_string());

        log::info!("description {:?}", self.description);
    }

    pub fn create_or_update_text(&self) -> String {
        match self.unit_id {
            Some(_) => STR_UPDATE.to_string(),
            None => STR_ADD_TO_COURSE.to_string(),
        }
    }
}
