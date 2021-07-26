use std::rc::Rc;
use dominator::{clone};
use shared::{api::{ApiEndpoint, endpoints}, domain::{image::*, meta::*, jig::module::body::Image}, error::{EmptyError, MetadataNotFound}, media::MediaLibrary};
use utils::prelude::*;
use web_sys::File;
use super::state::{BACKGROUND_NAME, State};
use super::super::upload::upload_image;

impl State {
    pub fn set_selected(&self, image: Image) {
        if let Some(on_select) = self.callbacks.on_select.as_ref() {
            on_select(image);
        }
    }
}

pub async fn get_styles() -> Vec<ImageStyle> {
    let res = api_with_auth::<MetadataResponse, (), ()>(
        &endpoints::meta::Get::PATH,
        endpoints::meta::Get::METHOD,
        None
    ).await;
    res.unwrap_ji().image_styles
}

pub fn get_background_id(styles: &Vec<ImageStyle>) -> ImageStyleId {
    styles
        .iter()
        .find(|s| s.display_name == BACKGROUND_NAME)
        .expect_ji(&format!("set \"{}\" in the database!", BACKGROUND_NAME))
        .id
        .clone()
}

pub fn search(state: Rc<State>) {
    let search_query = ImageSearchQuery {
        q: state.query.lock_ref().clone(),
        page: state.page.lock_ref().clone(),
        styles: state.selected_styles
            .borrow()
            .iter()
            .map(|style_id| style_id.clone())
            .collect(),
        kind: Some(ImageKind::Sticker),
        ..Default::default()
    };
    state.loader.load(clone!(state => async move {
        let res = api_with_auth::<ImageSearchResponse, EmptyError, _>(
            &endpoints::image::Search::PATH,
            endpoints::image::Search::METHOD,
            Some(search_query)
        ).await;
        match res {
            Ok(res) => {
                state.image_list
                    .lock_mut()
                    .replace_cloned(res.images.iter().map(|ir| ir.metadata.clone())
                    .collect());
            },
            Err(e) => {
                log::error!("{:#?}", e);
            }
        }
    }));
}

pub async fn upload_file(state: Rc<State>, file: File) {
    match api_with_auth::<CreateResponse, EmptyError, _>(endpoints::image::user::Create::PATH, endpoints::image::user::Create::METHOD, None::<()>).await {
        Ok(resp) => {
            let CreateResponse { id } = resp;

            match upload_image(id, MediaLibrary::User, &file, None).await {
                Ok(_) => {
                    state.set_selected(Image {id, lib: MediaLibrary::User}); 
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
            log::error!("error creating image db!")
        }
    }
}


