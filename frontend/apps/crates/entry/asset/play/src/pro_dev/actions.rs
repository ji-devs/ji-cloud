use dominator::clone;
use shared::{
    api::endpoints::pro_dev,
    domain::{
        asset::DraftOrLive,
        pro_dev::{ProDevGetDraftPath, ProDevGetLivePath},
    },
};
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;

use super::state::ProDevPlayer;

impl ProDevPlayer {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let pro_dev = match state.player_options.draft_or_live {
                DraftOrLive::Live => {
                    let pro_dev = {
                        pro_dev::GetLive::api_no_auth(ProDevGetLivePath(state.pro_dev_id), None).await
                    };

                    pro_dev
                },
                DraftOrLive::Draft => {
                    let pro_dev = {
                        pro_dev::GetDraft::api_no_auth(ProDevGetDraftPath(state.pro_dev_id), None).await
                    };

                    pro_dev
                },
            };

            match pro_dev {
                Ok(pro_dev) => {
                    if let Some(start_unit_id) = state.start_unit_id {
                        if let Some((index, _)) = pro_dev.pro_dev_data.units.iter().enumerate().find(|unit| {
                            unit.1.id == start_unit_id
                        }) {
                            state.active_unit.set_neq(Some(index));
                        };
                    }
                    state.pro_dev.set(Some(Rc::new(pro_dev)));
                },
                Err(_) => {
                    todo!();
                },
            }
        }));
    }
}
