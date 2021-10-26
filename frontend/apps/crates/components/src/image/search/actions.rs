use crate::image::search::state::{ImageSearchCheckboxKind, SearchMode, RECENT_COUNT};
use crate::image::tag::ImageTag;

use super::super::upload::upload_image;
use super::state::State;
use dominator::clone;
use futures::future::join;
use futures_signals::signal_vec::MutableVec;
use shared::api::endpoints::image;
use shared::domain::image::{
    recent::{UserRecentImageListRequest, UserRecentImageUpsertRequest},
    user::UserImageCreateRequest,
};
use shared::domain::meta::ImageTagIndex;
use shared::domain::search::WebImageSearchQuery;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{CreateResponse, ImageKind, ImageSearchQuery},
        jig::module::body::Image,
        meta::{ImageStyle, MetadataResponse},
    },
    error::EmptyError,
    media::MediaLibrary,
};
use url::Url;
use utils::web_image_resolver::ImageOrWeb;
use std::rc::Rc;
use std::str::FromStr;
use utils::prelude::*;
use web_sys::File;

impl State {
    pub fn set_selected(&self, image: Image) {
        if let Some(on_select) = self.callbacks.on_select.as_ref() {
            on_select(image.clone());
        }
        add_recent(&self, &image);
    }
}

pub fn on_web_image_click(state: Rc<State>, url: &str) {
    // TODO: why does the search request return strings but url is required here?
    let url = Url::from_str(url).unwrap_ji();

    state.loader.load(clone!(state => async move {

        let image = ImageOrWeb::web_to_image(url).await;
        state.set_selected(image);

    }));
}

pub async fn get_styles() -> Vec<ImageStyle> {
    let res = api_with_auth::<MetadataResponse, (), ()>(
        &endpoints::meta::Get::PATH,
        endpoints::meta::Get::METHOD,
        None,
    )
    .await;
    res.unwrap_ji().image_styles
}

pub fn search(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        match state.search_mode.get_cloned() {
            SearchMode::Sticker(_) => search_async(Rc::clone(&state)).await,
            SearchMode::Web(_) => search_async_web(Rc::clone(&state)).await,
        };
    }));
}

pub fn fetch_init_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let search = search_async(Rc::clone(&state));
        if state.options.recent {
            join(
                search,
                get_recent(Rc::clone(&state))
            ).await;
        } else {
            search.await;
        }
    }));
}

async fn search_async_web(state: Rc<State>) {
    let kind = match &state.options.checkbox_kind {
        Some(ImageSearchCheckboxKind::StickersFilter) if state.checkbox_checked.get() => Some(ImageKind::Sticker),
        _ => None,
    };

    let mut tags = state.options.tags.clone().unwrap_or_default();
    match &state.options.checkbox_kind {
        Some(ImageSearchCheckboxKind::BackgroundLayer1Filter) | Some(ImageSearchCheckboxKind::BackgroundLayer2Filter) => {
            let tag = match &state.options.checkbox_kind {
                Some(ImageSearchCheckboxKind::BackgroundLayer1Filter) => ImageTag::BackgroundLayer1,
                Some(ImageSearchCheckboxKind::BackgroundLayer2Filter) => ImageTag::BackgroundLayer2,
                _ => unreachable!(),
            };
            if state.checkbox_checked.get() {
                if !tags.contains(&tag) {
                    tags.push(tag);
                };
            } else {
                tags.retain(|t| t != &tag);
            };
        },
        _ => {},
    };

    let req = WebImageSearchQuery {
        q: state.query.lock_ref().clone(),
    };

    let res = endpoints::search::WebImageSearch::api_with_auth(Some(req)).await;

    match res {
        Ok(res) => {
            state.search_mode.set(SearchMode::Web(Rc::new(MutableVec::new_with_values(res.images))));
        },
        Err(e) => {
            log::error!("{:#?}", e);
        }
    }
}

async fn search_async(state: Rc<State>) {
    if state.user.borrow().is_none() {
        get_user(Rc::clone(&state)).await;
    }

    let affiliations = match &*state.user.borrow() {
        Some(user) => user.affiliations.clone(),
        None => unreachable!("User should exist"),
    };

    let kind = match &state.options.checkbox_kind {
        Some(ImageSearchCheckboxKind::StickersFilter) if state.checkbox_checked.get() => {
            Some(ImageKind::Sticker)
        }
        _ => None,
    };

    let mut tags = state.options.tags.clone().unwrap_or_default();
    match &state.options.checkbox_kind {
        Some(ImageSearchCheckboxKind::BackgroundLayer1Filter)
        | Some(ImageSearchCheckboxKind::BackgroundLayer2Filter) => {
            let tag = match &state.options.checkbox_kind {
                Some(ImageSearchCheckboxKind::BackgroundLayer1Filter) => ImageTag::BackgroundLayer1,
                Some(ImageSearchCheckboxKind::BackgroundLayer2Filter) => ImageTag::BackgroundLayer2,
                _ => unreachable!(),
            };
            if state.checkbox_checked.get() {
                if !tags.contains(&tag) {
                    tags.push(tag);
                };
            } else {
                tags.retain(|t| t != &tag);
            };
        }
        _ => {}
    };

    let search_query = ImageSearchQuery {
        q: state.query.lock_ref().clone(),
        page: state.page.lock_ref().clone(),
        styles: state
            .selected_styles
            .borrow()
            .iter()
            .map(|style_id| style_id.clone())
            .collect(),
        affiliations,
        kind,
        tags: tags.iter().map(|x| ImageTagIndex(x.as_index())).collect(),
        tags_priority: state
            .options
            .tags_priority
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|x| ImageTagIndex(x.as_index()))
            .collect(),
        ..Default::default()
    };

    // log::info!("{:?}", search_query);

    let res = endpoints::image::Search::api_with_auth(Some(search_query)).await;

    match res {
        Ok(res) => {
            let images: Vec<Image> = res.images.iter().map(|i| {
                Image {
                    id: i.metadata.id,
                    lib: MediaLibrary::Global,
                }
            }).collect();
            state.search_mode.set(SearchMode::Sticker(Rc::new(MutableVec::new_with_values(images))));

            // state.image_list.lock_mut().replace_cloned(res.images.iter().map(|i| {
            //     Image {
            //         id: i.metadata.id,
            //         lib: MediaLibrary::Global,
            //     }
            // }).collect());
        },
        Err(e) => {
            log::error!("{:#?}", e);
        }
    }
}

async fn get_user(state: Rc<State>) {
    match endpoints::user::Profile::api_with_auth(None).await {
        Err(_) => todo!(),
        Ok(user) => {
            *state.user.borrow_mut() = Some(user);
        }
    }
}

async fn get_recent(state: Rc<State>) {
    let req = UserRecentImageListRequest {
        limit: RECENT_COUNT,
    };

    match image::recent::List::api_with_auth(Some(req)).await {
        Err(_) => log::error!("Error getting recent images"),
        Ok(res) => {
            state.recent_list.lock_mut().replace_cloned(
                res.images
                    .iter()
                    .map(|i| Image {
                        id: i.id,
                        lib: i.library,
                    })
                    .collect(),
            );
        }
    };
}

pub fn add_recent(state: &State, image: &Image) {
    {
        let mut recent_list = state.recent_list.lock_mut();

        if let Some(index) = recent_list.iter().position(|i| i == image) {
            recent_list.remove(index);
        }

        recent_list.insert_cloned(0, image.clone());

        if recent_list.len() > RECENT_COUNT.into() {
            recent_list.pop();
        }
    }

    let req = UserRecentImageUpsertRequest {
        id: image.id.clone(),
        library: image.lib.clone(),
    };
    state.loader.load(async {
        let _ = image::recent::Put::api_with_auth(Some(req)).await;
    });
}

pub async fn upload_file(state: Rc<State>, file: File) {
    let req = UserImageCreateRequest {
        kind: ImageKind::Sticker,
    };

    match api_with_auth::<CreateResponse, EmptyError, _>(
        endpoints::image::user::Create::PATH,
        endpoints::image::user::Create::METHOD,
        Some(req),
    )
    .await
    {
        Ok(resp) => {
            let CreateResponse { id } = resp;

            match upload_image(id, MediaLibrary::User, &file, None).await {
                Ok(_) => {
                    state.set_selected(Image {
                        id,
                        lib: MediaLibrary::User,
                    });
                }
                Err(err) => {
                    if err.is_abort() {
                        log::info!("aborted!");
                    } else {
                        log::error!("got error! {:?}", err);
                    }
                }
            }
        }
        Err(_) => {
            log::error!("error creating image db!")
        }
    }
}
