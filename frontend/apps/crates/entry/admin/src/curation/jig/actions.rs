use std::rc::Rc;

use dominator::clone;
use shared::{domain::jig::{JigUpdateDraftDataRequest, JigUpdateAdminDataRequest}, error::EmptyError, api::{endpoints::{self, jig::JigAdminDataUpdate}, ApiEndpoint}};
use utils::prelude::api_with_auth_empty;

use super::state::CurationJig;

impl CurationJig {
    pub async fn save_draft(self: &Rc<Self>) {
        let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &self.jig_id.0.to_string());
        let req = self.jig.to_jig_update_request();
        let res = api_with_auth_empty::<EmptyError, JigUpdateDraftDataRequest>(
            &path,
            endpoints::jig::UpdateDraftData::METHOD,
            Some(req),
        ).await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub async fn save_admin_data(self: &Rc<Self>) {
        let path = endpoints::jig::JigAdminDataUpdate::PATH.replace("{id}", &self.jig_id.0.to_string());
        let req = self.jig.to_update_admin_data_request();
        let res = api_with_auth_empty::<EmptyError, JigUpdateAdminDataRequest>(
            &path,
            endpoints::jig::JigAdminDataUpdate::METHOD,
            Some(req),
        ).await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub async fn publish(self: &Rc<Self>) {
        let path = endpoints::jig::Publish::PATH.replace("{id}", &self.jig.id.0.to_string());
        let res = api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Publish::METHOD, None)
            .await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub fn save_and_publish(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.save_draft().await;
            state.save_admin_data().await;
            state.publish().await;
        }))
    }
}
