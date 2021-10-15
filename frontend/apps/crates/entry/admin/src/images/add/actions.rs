use super::state::*;
use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{image::*, meta::*},
    error::*,
    media::MediaLibrary,
};
use std::rc::Rc;
use utils::prelude::*;
use web_sys::File;

pub fn on_change(state: Rc<State>, value: String) {
    match value.as_ref() {
        "sticker" => {
            *state.kind.borrow_mut() = ImageKind::Sticker;
        }
        "canvas" => {
            *state.kind.borrow_mut() = ImageKind::Canvas;
        }
        _ => {
            log::info!("unknown value [{}]", value);
        }
    }
}

pub fn on_file(state: Rc<State>, file: File) {
    state.loader.load(clone!(state => async move {
        let meta_resp = api_no_auth::<MetadataResponse, (), ()>(&endpoints::meta::Get::PATH, endpoints::meta::Get::METHOD, None).await.expect_ji("couldn't get meta response!");

        let affiliations = meta_resp.affiliations
            .iter()
            .map(|x| x.id)
            .collect();

        let age_ranges = meta_resp.age_ranges
            .iter()
            .map(|x| x.id)
            .collect();

        let req = ImageCreateRequest {
            name: "".to_string(),
            description: "".to_string(),
            is_premium: false,
            publish_at: None,
            tags: Vec::new(),
            styles: Vec::new(),
            age_ranges,
            affiliations,
            categories: Vec::new(),
            kind: state.kind.borrow().clone()
        };

        match api_with_auth::<CreateResponse, MetadataNotFound, _>(endpoints::image::Create::PATH, endpoints::image::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                let CreateResponse { id} = resp;
               
                match upload_image(id, MediaLibrary::Global, &file, None).await {
                    Ok(_) => {
                        let route:String = Route::Admin(AdminRoute::ImageMeta(id, true)).into();
                        dominator::routing::go_to_url(&route);
                    },
                    Err(err) => {
                        if err.is_abort() {
                            log::info!("aborted!");
                        } else {
                            log::error!("got error! {:?}", err);
                        }
                    }
                }
            },
            Err(_) => {
                log::error!("error creating image id!")
            }
        }
    }))
}

async fn _upload_file(_file: File, id: ImageId, session_uri: &str) {
    log::info!("Uploading {} to {}", id.0.to_string(), session_uri);
}
