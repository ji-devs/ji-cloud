use shared::{
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
use components::firebase;

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

                let req = ImageUploadRequest {
                    file_size: file.size() as usize
                };

                let path = endpoints::image::Upload::PATH.replace("{id}",&id.0.to_string());

                match api_with_auth::<ImageUploadResponse, EmptyError, _>(&path, endpoints::image::Upload::METHOD, Some(req)).await {
                    Ok(resp) => {
                        let ImageUploadResponse {session_uri} = resp;
                      
                        //Before we upload, setup the firestore listener 
                        //so we get all the status updates
                        *state.upload_listener.borrow_mut() = Some(firebase::add_upload_listener(&id.0.to_string(), clone!(state => move |status| {
                            if status.ready {
                                log::info!("READY!");
                                state.upload_listener.borrow_mut().take();
                                let route:String = Route::Admin(AdminRoute::ImageMeta(id, true)).into();
                                dominator::routing::go_to_url(&route);
                            } else if status.processing {
                                log::info!("{} has started processing, waiting to finalize...", id.0.to_string());
                            } else {
                                log::info!("{:?}", status);
                            }
                        })));

                        //upload to GCS
                        match upload_file_gcs(&session_uri, &file).await {
                            Ok(_) => {
                                log::info!("{} uploaded, waiting for processing to start...", id.0.to_string());
                            },
                            Err(_) => {
                                //Something went wrong, clear the firestore listener
                                state.upload_listener.borrow_mut().take();
                            },
                        }
                    },

                    Err(err) => {
                        log::error!("error getting image upload uri!")
                    }
                }
            },
            Err(err) => {
                log::error!("error creating image id!")
            }
        }
    }))
}

async fn upload_file(file: File, id: ImageId, session_uri: &str) {
    log::info!("Uploading {} to {}", id.0.to_string(), session_uri);
}
