use crate::schools::details::state::{CurrentAction, SchoolDetails};
use dominator::clone;
use shared::api::endpoints;
use shared::domain::admin::{InviteSchoolUsersRequest, VerifySchoolNameRequest};
use shared::domain::{
    admin::{AdminVerifySchoolNamePath, InviteSchoolUsersPath},
    billing::SchoolAccountPath,
};
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;
use utils::routes::AdminSchoolsRoute;

impl SchoolDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.parent.loader.load(clone!(state => async move {
            match endpoints::account::GetSchoolAccount::api_with_auth(
                SchoolAccountPath(state.school_id),
                None,
            )
            .await
            {
                Err(_) => todo!(),
                Ok(school_account) => {
                    state.school.set(Some(school_account.school));
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
            match endpoints::admin::VerifySchoolName::api_with_auth_empty(AdminVerifySchoolNamePath(), Some(VerifySchoolNameRequest {
                school_name_id: school.school_name.id,
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
}
