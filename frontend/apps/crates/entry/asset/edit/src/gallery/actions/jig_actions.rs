use super::super::state::Gallery;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        jig::{
            JigBrowsePath, JigBrowseQuery, JigClonePath, JigDeletePath, JigGetDraftPath, JigId,
            JigSearchPath, JigSearchQuery,
        },
    },
    error::IntoAnyhow,
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_jigs(
    state: &Rc<Gallery>,
    is_published: Option<bool>,
) -> Result<(Vec<Asset>, u64), ()> {
    let req = JigBrowseQuery {
        page: Some(*state.next_page.lock_ref()),
        is_published,
        author_id: Some(UserOrMe::Me),
        draft_or_live: Some(DraftOrLive::Draft),
        ..Default::default()
    };

    endpoints::jig::Browse::api_with_auth(JigBrowsePath(), Some(req))
        .await
        .map(|res| {
            let assets = res.jigs.into_iter().map(|jig| jig.into()).collect();
            (assets, res.total_jig_count)
        })
        .map_err(|_| ())
}

pub async fn search_jigs(q: String, is_published: Option<bool>) -> Result<Vec<Asset>, ()> {
    let req = JigSearchQuery {
        q,
        is_published,
        author_id: Some(UserOrMe::Me),
        ..Default::default()
    };

    endpoints::jig::Search::api_with_auth(JigSearchPath(), Some(req))
        .await
        .map(|resp| resp.jigs.into_iter().map(|jig| jig.into()).collect())
        .map_err(|_| ())
}

pub async fn copy_jig(jig_id: JigId) -> Result<Asset, ()> {
    match endpoints::jig::Clone::api_with_auth(JigClonePath(jig_id), None).await {
        Ok(resp) => endpoints::jig::GetDraft::api_with_auth(JigGetDraftPath(resp.id), None)
            .await
            .map(|resp| {
                let asset: Asset = resp.into();
                asset
            })
            .map_err(|_| ()),
        Err(_) => Err(()),
    }
}

pub async fn delete_jig(jig_id: JigId) -> anyhow::Result<()> {
    endpoints::jig::Delete::api_with_auth(JigDeletePath(jig_id), None)
        .await
        .into_anyhow()
}
