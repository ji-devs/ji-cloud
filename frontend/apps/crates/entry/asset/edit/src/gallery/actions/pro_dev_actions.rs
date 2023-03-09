use super::super::state::Gallery;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        pro_dev::{
            ProDevBrowsePath, ProDevBrowseQuery, ProDevCreatePath,
            ProDevCreateRequest, ProDevDeletePath, ProDevGetDraftPath, ProDevId, ProDevSearchPath,
            ProDevSearchQuery,
        },
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
    },
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_pro_devs(
    state: &Rc<Gallery>,
    is_published: Option<bool>,
) -> Result<(Vec<Asset>, u64), ()> {
    let req = ProDevBrowseQuery {
        page: Some(*state.next_page.lock_ref()),
        is_published,
        author_id: Some(UserOrMe::Me),
        draft_or_live: Some(DraftOrLive::Draft),
        ..Default::default()
    };

    endpoints::pro_dev::Browse::api_with_auth(ProDevBrowsePath(), Some(req))
        .await
        .map(|res| {
            let assets = res
                .pro_devs
                .into_iter()
                .map(|pro_dev| pro_dev.into())
                .collect();
            (assets, res.total_pro_dev_count)
        })
        .map_err(|_| ())
}

pub async fn search_pro_devs(q: String, is_published: Option<bool>) -> Result<Vec<Asset>, ()> {
    let req = ProDevSearchQuery {
        q,
        is_published,
        author_id: Some(UserOrMe::Me),
        ..Default::default()
    };

    endpoints::pro_dev::Search::api_with_auth(ProDevSearchPath(), Some(req))
        .await
        .map(|resp| {
            resp.pro_devs
                .into_iter()
                .map(|pro_dev| pro_dev.into())
                .collect()
        })
        .map_err(|_| ())
}

pub async fn create_pro_dev() {
    let req = ProDevCreateRequest {
        ..Default::default()
    };

    match endpoints::pro_dev::Create::api_with_auth(ProDevCreatePath(), Some(req)).await {
        Ok(resp) => {
            add_cover(&resp.id).await;
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
                resp.id,
                ProDevEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

async fn add_cover(pro_dev_id: &ProDevId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*pro_dev_id).into(),
    };


    match endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await {
        Ok(_) => {}
        Err(_) => {
            todo!()
        }
    }
}

pub async fn delete_pro_dev(pro_dev_id: ProDevId) -> anyhow::Result<()> {
    endpoints::pro_dev::Delete::api_with_auth_empty(ProDevDeletePath(pro_dev_id), None)
        .await
        .map(|_| ())
}
