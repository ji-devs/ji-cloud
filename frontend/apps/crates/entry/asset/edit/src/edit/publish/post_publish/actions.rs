use std::rc::Rc;

use shared::{
    api::endpoints,
    domain::{
        jig::{JigCreatePath, JigCreateRequest},
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
        playlist::{PlaylistCreatePath, PlaylistCreateRequest, PlaylistId},
        pro_dev::{ProDevCreatePath, ProDevCreateRequest, ProDevId},
        resource::{ResourceCreatePath, ResourceCreateRequest, ResourceId},
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{
        AssetEditRoute, AssetRoute, JigEditRoute, PlaylistEditRoute, ProDevEditRoute,
        ResourceEditRoute, Route,
    },
    unwrap::UnwrapJiExt,
};

use super::state::PostPublish;

impl PostPublish {
    pub fn create_jig(self: &Rc<Self>) {
        let state = self;
        state.loader.load(async move {
            let req = JigCreateRequest::default();

            let resp = endpoints::jig::Create::api_with_auth(JigCreatePath(), Some(req))
                .await
                .unwrap_ji();
            let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                resp.id,
                JigEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&url);
        });
    }
    pub fn create_resource(self: &Rc<Self>) {
        let state = self;
        state.loader.load(async move {
            let req = ResourceCreateRequest::default();

            let resp = endpoints::resource::Create::api_with_auth(ResourceCreatePath(), Some(req))
                .await
                .unwrap_ji();
            add_resource_cover(&resp.id).await.unwrap_ji();
            let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
                resp.id,
                ResourceEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&url);
        });
    }
    pub fn create_playlist(self: &Rc<Self>) {
        let state = self;
        state.loader.load(async move {
            let req = PlaylistCreateRequest::default();

            let resp = endpoints::playlist::Create::api_with_auth(PlaylistCreatePath(), Some(req))
                .await
                .unwrap_ji();
            add_playlist_cover(&resp.id).await.unwrap_ji();
            let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                resp.id,
                PlaylistEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&url);
        });
    }
    pub fn create_pro_dev(self: &Rc<Self>) {
        let state = self;
        state.loader.load(async move {
            let req = ProDevCreateRequest::default();

            let resp = endpoints::pro_dev::Create::api_with_auth(ProDevCreatePath(), Some(req))
                .await
                .unwrap_ji();
            add_pro_dev_cover(&resp.id).await.unwrap_ji();
            let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
                resp.id,
                ProDevEditRoute::Landing,
            )))
            .into();
            dominator::routing::go_to_url(&url);
        });
    }
}

async fn add_resource_cover(resource_id: &ResourceId) -> anyhow::Result<()> {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*resource_id).into(),
    };

    endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await?;

    Ok(())
}

async fn add_playlist_cover(playlist_id: &PlaylistId) -> anyhow::Result<()> {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*playlist_id).into(),
    };

    endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await?;

    Ok(())
}

async fn add_pro_dev_cover(pro_dev_id: &ProDevId) -> anyhow::Result<()> {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*pro_dev_id).into(),
    };

    endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await?;

    Ok(())
}
