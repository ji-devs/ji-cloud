use std::rc::Rc;

use dominator::clone;
use futures::future::try_join_all;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        pro_dev::{ProDevGetDraftPath, ProDevGetLivePath, unit::{ProDevUnit, ProDevUnitId, GetProDevUnitLivePath}},
        meta::GetMetadataPath,
    },
};
use utils::{
    iframe::{AssetPlayerToPlayerPopup, IframeAction, IframeMessageExt},
    prelude::ApiEndpointExt,
    unwrap::UnwrapJiExt,
};

use super::state::ProDevPlayer;

impl ProDevPlayer {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_pro_dev(),
                state.load_resource_types(),
            );
        }));
    }

    async fn load_pro_dev(self: &Rc<Self>) {
        let state = self;
        let pro_dev = match state.player_options.draft_or_live {
            DraftOrLive::Live => {
                endpoints::pro_dev::GetLive::api_no_auth(ProDevGetLivePath(state.pro_dev_id), None)
                    .await
            }
            DraftOrLive::Draft => {
                endpoints::pro_dev::GetDraft::api_no_auth(ProDevGetDraftPath(state.pro_dev_id), None)
                    .await
            }
        };

        match pro_dev {
            Ok(pro_dev) => {
                let unit_ids = pro_dev.pro_dev_data.units.clone().into_iter().map(|x| x.id).collect();
                state.pro_dev.set(Some(pro_dev));
                state.load_units(unit_ids).await;
            }
            Err(_) => {
                todo!();
            }
        }
    }

    async fn load_resource_types(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.resource_types.set(meta.resource_types);
            }
        };
    }

    async fn load_units(self: &Rc<Self>, unit_ids: Vec<ProDevUnitId>) {
        let units = try_join_all(unit_ids.iter().map(|unit_id| self.load_unit(unit_id)))
            .await
            .unwrap_ji();

        self.units.set(units);
    }

    async fn load_unit(self: &Rc<Self>, unit_id: &ProDevUnitId) -> Result<ProDevUnit, ()> {
        endpoints::pro_dev::unit::GetLive::api_no_auth(GetProDevUnitLivePath(self.pro_dev_id, unit_id.clone()), None)
            .await
            .map_err(|_| ())
    }

    pub fn play_unit(self: &Rc<Self>, unit_id: ProDevUnitId) {
        self.active_unit.set(Some(unit_id));
        let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(false))
            .try_post_message_to_parent();
    }

    pub fn done_playing_unit(self: &Rc<Self>) {
        self.active_unit.set(None);
        let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(true))
            .try_post_message_to_parent();
    }
}
