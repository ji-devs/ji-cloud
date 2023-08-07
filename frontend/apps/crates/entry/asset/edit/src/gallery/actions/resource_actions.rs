use super::super::state::Gallery;

use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        resource::{
            ResourceBrowsePath, ResourceBrowseQuery, ResourceClonePath, ResourceDeletePath,
            ResourceGetDraftPath, ResourceId, ResourceSearchPath, ResourceSearchQuery,
        },
    },
    error::IntoAnyhow,
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_resources(
    state: &Rc<Gallery>,
    is_published: Option<bool>,
) -> Result<(Vec<Asset>, u64), ()> {
    let req = ResourceBrowseQuery {
        page: Some(*state.next_page.lock_ref()),
        is_published,
        author_id: Some(UserOrMe::Me),
        draft_or_live: Some(DraftOrLive::Draft),
        ..Default::default()
    };

    endpoints::resource::Browse::api_with_auth(ResourceBrowsePath(), Some(req))
        .await
        .map(|res| {
            let assets = res
                .resources
                .into_iter()
                .map(|resource| resource.into())
                .collect();
            (assets, res.total_resource_count)
        })
        .map_err(|_| ())
}

pub async fn search_resources(q: String, is_published: Option<bool>) -> Result<Vec<Asset>, ()> {
    let req = ResourceSearchQuery {
        q,
        is_published,
        author_id: Some(UserOrMe::Me),
        ..Default::default()
    };

    endpoints::resource::Search::api_with_auth(ResourceSearchPath(), Some(req))
        .await
        .map(|resp| {
            resp.resources
                .into_iter()
                .map(|resource| resource.into())
                .collect()
        })
        .map_err(|_| ())
}

pub async fn copy_resource(resource_id: ResourceId) -> Result<Asset, ()> {
    match endpoints::resource::Clone::api_with_auth(ResourceClonePath(resource_id), None).await {
        Ok(resp) => {
            endpoints::resource::GetDraft::api_with_auth(ResourceGetDraftPath(resp.id), None)
                .await
                .map(|resp| {
                    let asset: Asset = resp.into();
                    asset
                })
                .map_err(|_| ())
        }
        Err(_) => Err(()),
    }
}

pub async fn delete_resource(resource_id: ResourceId) -> anyhow::Result<()> {
    endpoints::resource::Delete::api_with_auth(ResourceDeletePath(resource_id), None)
        .await
        .map(|_| ())
        .into_anyhow()
}
