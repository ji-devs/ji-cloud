use shared::{
    api::endpoints,
    domain::{
        course::{CourseCreatePath, CourseCreateRequest},
        jig::{JigCreatePath, JigCreateRequest},
        resource::{ResourceCreatePath, ResourceCreateRequest},
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, ResourceEditRoute, Route},
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
        let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::Resource(
            resp.id,
            ResourceEditRoute::Landing,
        )))
        .into();
        dominator::routing::go_to_url(&url);
    });
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

// pub fn create_pro_dev() {
//     spawn_local(async move {
//         let req = ProDevCreateRequest::default();

//         let resp = endpoints::pro_dev::Create::api_with_auth(ProDevCreatePath(), Some(req))
//             .await
//             .unwrap_ji();
//         let url: String = Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
//             resp.id,
//             ProDevEditRoute::Landing,
//         )))
//         .into();
//         dominator::routing::go_to_url(&url);
//     });
// }
