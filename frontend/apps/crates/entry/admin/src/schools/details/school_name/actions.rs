use crate::schools::details::school_name::state::SchoolNameState;
use dominator::clone;
use shared::api::endpoints;
use shared::domain::admin::{SchoolNamesPath, SetInternalSchoolNamePath, UpdateSchoolNamePath};
use shared::domain::billing::SchoolNameId;
use std::rc::Rc;
use utils::{error_ext::ErrorExt, prelude::ApiEndpointExt, routes::AdminSchoolsRoute};

impl SchoolNameState {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.parent.parent.loader.load(clone!(state => async move {
            match endpoints::admin::GetSchoolNames::api_with_auth(
                SchoolNamesPath(),
                None,
            )
            .await
            {
                Err(_) => todo!(),
                Ok(school_names) => state.school_names.set(Some(school_names))
            }
        }));
    }

    pub fn update_school_name(self: &Rc<Self>) {
        let state = Rc::clone(self);
        let school_name = self.current_name.get().unwrap();

        state.parent.parent.loader.load(clone!(state => async move {
            if endpoints::admin::UpdateSchoolName::api_with_auth(
                UpdateSchoolNamePath(school_name.id),
                Some(school_name.name.trim().to_string().into())
            ).await.toast_on_err().is_ok() {
                state.parent.parent.navigate_to(AdminSchoolsRoute::Table)
            }
        }));
    }

    pub fn create_school_name(self: &Rc<Self>) {
        let state = Rc::clone(self);
        let school_name = self.new_name.get().unwrap();

        state.parent.parent.loader.load(clone!(state => async move {
            if let Ok(id) = endpoints::admin::CreateSchoolName::api_with_auth(
                SchoolNamesPath(),
                Some(school_name.trim().to_string().into())
            ).await.toast_on_err() {
                if endpoints::admin::SetInternalSchoolName::api_with_auth(
                    SetInternalSchoolNamePath(state.parent.school_id),
                    Some(id)
                ).await.toast_on_err().is_ok() {
                    state.parent.parent.navigate_to(AdminSchoolsRoute::Table)
                }
            }
        }));
    }

    pub fn change_internal_school_name(self: &Rc<Self>, school_name_id: SchoolNameId) {
        let state = Rc::clone(self);

        state.parent.parent.loader.load(clone!(state => async move {
            if endpoints::admin::SetInternalSchoolName::api_with_auth(
                SetInternalSchoolNamePath(state.parent.school_id),
                Some(school_name_id)
            ).await.toast_on_err().is_ok() {
                state.parent.parent.navigate_to(AdminSchoolsRoute::Table)
            }
        }));
    }
}
