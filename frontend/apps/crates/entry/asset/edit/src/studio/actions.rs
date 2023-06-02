use shared::{
    api::endpoints,
    domain::{
        asset::AssetId,
        course::{CourseCreatePath, CourseCreateRequest},
        jig::{JigCreatePath, JigCreateRequest},
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
        playlist::{PlaylistCreatePath, PlaylistCreateRequest},
        resource::{ResourceCreatePath, ResourceCreateRequest},
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{
        AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, PlaylistEditRoute,
        ResourceEditRoute, Route,
    },
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;

pub fn create_jig() {
    spawn_local(async move {
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

pub fn create_resource() {
    spawn_local(async move {
        let req = ResourceCreateRequest::default();

        let resp = endpoints::resource::Create::api_with_auth(ResourceCreatePath(), Some(req))
            .await
            .unwrap_ji();
        add_playlist_or_resource_cover(resp.id.into()).await;
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
            resp.id,
            ResourceEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}

pub fn create_playlist() {
    spawn_local(async move {
        let req = PlaylistCreateRequest::default();

        let resp = endpoints::playlist::Create::api_with_auth(PlaylistCreatePath(), Some(req))
            .await
            .unwrap_ji();
        add_playlist_or_resource_cover(resp.id.into()).await;
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
            resp.id,
            PlaylistEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}

async fn add_playlist_or_resource_cover(asset_id: AssetId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: asset_id,
    };

    endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req))
        .await
        .unwrap_ji();
}

pub fn create_course() {
    spawn_local(async move {
        let req = CourseCreateRequest::default();

        let resp = endpoints::course::Create::api_with_auth(CourseCreatePath(), Some(req))
            .await
            .unwrap_ji();
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
            resp.id,
            CourseEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
}
