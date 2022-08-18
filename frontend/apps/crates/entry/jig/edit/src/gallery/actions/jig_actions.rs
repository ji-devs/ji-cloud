use super::super::state::Gallery;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCreateRequest, JigId, JigResponse, JigSearchQuery,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
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

    api_with_auth::<JigBrowseResponse, EmptyError, _>(
        endpoints::jig::Browse::PATH,
        endpoints::jig::Browse::METHOD,
        Some(req),
    )
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

    endpoints::jig::Search::api_with_auth(Some(req))
        .await
        .map(|resp| resp.jigs.into_iter().map(|jig| jig.into()).collect())
        .map_err(|_| ())
}

pub async fn create_jig() {
    let req = JigCreateRequest::default();

    match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(
        endpoints::jig::Create::PATH,
        endpoints::jig::Create::METHOD,
        Some(req),
    )
    .await
    {
        Ok(resp) => {
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                resp.id,
                JigEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

pub async fn copy_jig(jig_id: JigId) -> Result<Asset, ()> {
    let path = endpoints::jig::Clone::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<CreateResponse<JigId>, EmptyError, ()>(
        &path,
        endpoints::jig::Clone::METHOD,
        None,
    )
    .await
    {
        Ok(resp) => {
            let path = endpoints::jig::GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
            api_with_auth::<JigResponse, EmptyError, ()>(
                &path,
                endpoints::jig::GetDraft::METHOD,
                None,
            )
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

pub async fn delete_jig(jig_id: JigId) -> Result<(), ()> {
    let path = endpoints::jig::Delete::PATH.replace("{id}", &jig_id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Delete::METHOD, None)
        .await
        .map(|_| ())
        .map_err(|_| ())
}
