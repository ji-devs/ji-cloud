use shared::{
    media::MediaLibrary,
    domain::image::*,
    error::*,
    api::{ApiEndpoint, endpoints},
};
use utils::{
    prelude::*,
    routes::*
};
use dominator::clone;
use super::state::*;
use std::rc::Rc;
use web_sys::File;
use components::image::upload::{upload_image};

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
            tags: Vec::new(),
            styles: Vec::new(),
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
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

async fn upload_file(file: File, id: ImageId, session_uri: &str) {
    log::info!("Uploading {} to {}", id.0.to_string(), session_uri);
}
