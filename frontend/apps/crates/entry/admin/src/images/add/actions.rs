use shared::{
    domain::image::*,
    error::*,
    api::{ApiEndpoint, endpoints},
};
use utils::{
    fetch::{api_with_auth, api_upload_file},
    routes::*
};
use dominator::clone;
use super::state::*;
use std::rc::Rc;
use web_sys::File;

pub fn on_change(state: Rc<State>, value: String) {
    match value.as_ref() {
        "sticker" => {
            *state.kind.borrow_mut() = ImageKind::Sticker;
        },
        "canvas" => {
            *state.kind.borrow_mut() = ImageKind::Canvas;
        },
        _ => {
            log::info!("unknown value [{}]", value);
        }
    }
}

pub fn on_file(state: Rc<State>, file: File) {
    state.loader.load(clone!(state => async move {
        let req = ImageCreateRequest {
            name: "".to_string(),
            description: "".to_string(),
            is_premium: false,
            publish_at: None,
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            categories: Vec::new(),
            kind: state.kind.borrow().clone()
        };

        match api_with_auth::<CreateResponse, MetadataNotFound, _>(endpoints::image::Create::PATH, endpoints::image::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                let CreateResponse { id} = resp;

                let path = endpoints::image::Upload::PATH.replace("{id}",&id.0.to_string());
                match api_upload_file(&path, &file, endpoints::image::Upload::METHOD).await {
                    Ok(_) => {
                        let route:String = Route::Admin(AdminRoute::ImageMeta(id, None)).into();
                        dominator::routing::go_to_url(&route);
                    },
                    Err(_) => {
                        log::error!("error uploading!");
                    }
                }
            },
            Err(_) => {
                log::error!("error creating image db!")
            }
        }
    }))
}
