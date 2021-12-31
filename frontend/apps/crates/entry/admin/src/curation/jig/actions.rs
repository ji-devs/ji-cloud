use std::rc::Rc;

use dominator::clone;
use shared::{domain::jig::JigUpdateDraftDataRequest, error::EmptyError, api::{endpoints, ApiEndpoint}};
use utils::prelude::api_with_auth_empty;

use super::state::CurationJig;

impl CurationJig {
    // async fn save(self: &Rc<Self>) -> Result<(), ()> {
    //     let state = self;
    //     let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &state.jig_id.0.to_string());
    //     let req = state.jig.to_jig_update_request();
    //     api_with_auth_empty::<EmptyError, JigUpdateDraftDataRequest>(
    //         &path,
    //         endpoints::jig::UpdateDraftData::METHOD,
    //         Some(req),
    //     )
    //         .await
    //         .map_err(|_| ())?;

    //     // let path = jig::Publish::PATH.replace("{id}", &state.jig.id.0.to_string());
    //     // api_with_auth_empty::<EmptyError, ()>(&path, jig::Publish::METHOD, None)
    //     //     .await
    //     //     .map_err(|_| ())?;

    //     Ok(())
    // }

    pub fn save_draft(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &state.jig_id.0.to_string());
            let req = state.jig.to_jig_update_request();
            let res = api_with_auth_empty::<EmptyError, JigUpdateDraftDataRequest>(
                &path,
                endpoints::jig::UpdateDraftData::METHOD,
                Some(req),
            ).await;
            match res {
                Ok(res) => res,
                Err(_) => todo!(),
            }
        }))
    }

    pub fn publish(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let path = endpoints::jig::Publish::PATH.replace("{id}", &state.jig.id.0.to_string());
            let res = api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Publish::METHOD, None)
                .await;
            match res {
                Ok(res) => res,
                Err(_) => todo!(),
            }
        }))
    }
}
