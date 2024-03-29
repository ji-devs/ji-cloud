use crate::schools::details::state::{CurrentAction, SchoolDetails};
use dominator::clone;
use shared::api::endpoints;
use shared::domain::admin::{
    AdminSchoolAccountPath, InviteSchoolUsersRequest, RemoveUserFromSchoolPath, VerifySchoolRequest,
};
use shared::domain::admin::{AdminVerifySchoolPath, InviteSchoolUsersPath};
use shared::domain::billing::SchoolAccountPath;
use shared::domain::user::UserId;
use std::rc::Rc;
use utils::error_ext::ErrorExt;
use utils::prelude::ApiEndpointExt;
use utils::routes::AdminSchoolsRoute;

impl SchoolDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.parent.loader.load(clone!(state => async move {
            match endpoints::admin::GetAdminSchoolAccount::api_with_auth(
                AdminSchoolAccountPath(state.school_id),
                None,
            )
            .await
            {
                Err(_) => todo!(),
                Ok(school_account) => {
                    state.school.set(Some(school_account.school.into()));
                    state.account.set(Some(school_account.account));
                    state.users.lock_mut()
                        .replace_cloned(school_account.users.into_iter().map(|user| Rc::new(user))
                        .collect());
                }
            }
        }));
    }

    pub fn invite_school_users(self: &Rc<Self>, data: String) {
        let state = Rc::clone(self);

        state.current_action.set(CurrentAction::AddingUsers);
        state.parent.loader.load(clone!(state => async move {
            let req = InviteSchoolUsersRequest {
                school_id: state.school_id,
                data,
            };

            match endpoints::admin::InviteUsers::api_with_auth(InviteSchoolUsersPath(), Some(req)).await {
                Err(_) => todo!(),
                Ok(response) => {
                    state.current_action.set(CurrentAction::Results(response.failures));
                    state.load_data()
                }
            }
        }));
    }

    pub fn set_verified(self: &Rc<Self>) {
        let state = Rc::clone(self);

        if let Some(school) = state.school.get_cloned() {
            state.parent.loader.load(clone!(state => async move {
                match endpoints::admin::VerifySchool::api_with_auth(AdminVerifySchoolPath(), Some(VerifySchoolRequest {
                    school_id: school.id,
                    verified: true,
                })).await {
                    Err(error) => {
                        log::error!("Error: {error:?}");
                    },
                    Ok(_) => state.parent.navigate_to(AdminSchoolsRoute::Table),
                }
            }));
        }
    }

    pub fn save_school(self: &Rc<Self>) {
        let state = Rc::clone(self);

        if let Some(school) = state.school.get_cloned() {
            state.parent.loader.load(clone!(state => async move {
                match endpoints::account::UpdateSchoolAccount::api_with_auth(SchoolAccountPath(school.id), Some(school.into())).await {
                    Err(error) => {
                        log::error!("Error: {error:?}");
                    },
                    Ok(_) => state.parent.navigate_to(AdminSchoolsRoute::Table),
                }
            }));
        }
    }

    pub fn remove_user_from_school(self: &Rc<Self>, user_id: UserId) {
        let state = Rc::clone(self);

        if let Some(school) = state.school.get_cloned() {
            state.parent.loader.load(clone!(state => async move {
                let _ = endpoints::admin::RemoveUserFromSchool::api_with_auth(RemoveUserFromSchoolPath(school.id), Some(user_id))
                    .await
                    .toast_on_err();

                match endpoints::admin::GetAdminSchoolAccount::api_with_auth(
                    AdminSchoolAccountPath(state.school_id),
                    None,
                )
                .await
                .toast_on_err()
                {
                    Err(_) => todo!(),
                    Ok(school_account) => {
                        state.school.set(Some(school_account.school.into()));
                        state.account.set(Some(school_account.account));
                        state.users.lock_mut()
                            .replace_cloned(school_account.users.into_iter().map(|user| Rc::new(user))
                            .collect());
                    }
                }
            }));
        }
    }
}
