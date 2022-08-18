use super::super::state::Gallery;
use components::module::_common::prelude::ModuleId;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        module::{ModuleBody, ModuleCreateRequest, ModuleKind},
        resource::{
            ResourceBrowseQuery, ResourceBrowseResponse, ResourceCreateRequest, ResourceId,
            ResourceSearchQuery,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
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

    api_with_auth::<ResourceBrowseResponse, EmptyError, _>(
        endpoints::resource::Browse::PATH,
        endpoints::resource::Browse::METHOD,
        Some(req),
    )
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

    endpoints::resource::Search::api_with_auth(Some(req))
        .await
        .map(|resp| {
            resp.resources
                .into_iter()
                .map(|resource| resource.into())
                .collect()
        })
        .map_err(|_| ())
}

pub async fn create_resource() {
    let req = ResourceCreateRequest::default();

    match api_with_auth::<CreateResponse<ResourceId>, MetadataNotFound, _>(
        endpoints::resource::Create::PATH,
        endpoints::resource::Create::METHOD,
        Some(req),
    )
    .await
    {
        Ok(resp) => {
            add_cover(&resp.id).await;
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                resp.id,
                ResourceEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

async fn add_cover(resource_id: &ResourceId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*resource_id).into(),
    };

    // let path = endpoints::module::Create::PATH.replace("{id}", &resource_id.0.to_string());

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

pub async fn copy_resource(_resource_id: ResourceId) -> Result<Asset, ()> {
    todo!();
    // let path = endpoints::resource::Clone::PATH.replace("{id}", &resource_id.0.to_string());

    // match api_with_auth::<CreateResponse<ResourceId>, EmptyError, ()>(
    //     &path,
    //     endpoints::resource::Clone::METHOD,
    //     None,
    // )
    // .await
    // {
    //     Ok(resp) => {
    //         let path = endpoints::resource::GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
    //         api_with_auth::<ResourceResponse, EmptyError, ()>(
    //             &path,
    //             endpoints::resource::GetDraft::METHOD,
    //             None,
    //         )
    //         .await
    //         .map(|resp| {
    //             let asset: Asset = resp.into();
    //             asset
    //         })
    //         .map_err(|_| ())
    //     }
    //     Err(_) => Err(()),
    // }
}

pub async fn delete_resource(resource_id: ResourceId) -> Result<(), ()> {
    let path = endpoints::resource::Delete::PATH.replace("{id}", &resource_id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, endpoints::resource::Delete::METHOD, None)
        .await
        .map(|_| ())
        .map_err(|_| ())
}
