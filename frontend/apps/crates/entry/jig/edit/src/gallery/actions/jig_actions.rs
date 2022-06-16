use super::super::state::Gallery;
use components::module::_common::prelude::ModuleId;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCreateRequest, JigFocus, JigId, JigResponse,
            JigSearchQuery,
        },
        module::{ModuleBody, ModuleCreateRequest, ModuleKind},
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
        jig_focus: Some(state.get_jig_focus()),
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
        jig_focus: Some(JigFocus::Modules),
        ..Default::default()
    };

    endpoints::jig::Search::api_with_auth(Some(req))
        .await
        .map(|resp| resp.jigs.into_iter().map(|jig| jig.into()).collect())
        .map_err(|_| ())
}

pub async fn create_jig(jig_focus: JigFocus) {
    let req = JigCreateRequest {
        jig_focus,
        ..Default::default()
    };

    match api_with_auth::<CreateResponse<JigId>, MetadataNotFound, _>(
        endpoints::jig::Create::PATH,
        endpoints::jig::Create::METHOD,
        Some(req),
    )
    .await
    {
        Ok(resp) => {
            if jig_focus.is_resources() {
                add_cover(&resp.id).await;
            }
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                resp.id,
                JigFocus::Modules,
                JigEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

async fn add_cover(jig_id: &JigId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*jig_id).into(),
    };

    // let path = endpoints::module::Create::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<CreateResponse<ModuleId>, EmptyError, _>(
        endpoints::module::Create::PATH,
        endpoints::module::Create::METHOD,
        Some(req),
    )
    .await
    {
        Ok(_) => {}
        Err(_) => {
            todo!()
        }
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
