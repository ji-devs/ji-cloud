use crate::firebase::wait_for_upload_ready;
use crate::image::search::state::{ImageSearchKind, NextPage, SearchMode, RECENT_COUNT};
use crate::image::tag::ImageTag;

use super::super::upload::upload_image;
use super::state::State;
use dominator::clone;
use futures::future::join;
use futures_signals::signal_vec::MutableVec;
use shared::api::endpoints::image;
use shared::domain::image::ImageId;
use shared::domain::image::{
    recent::{UserRecentImageListRequest, UserRecentImageUpsertRequest},
    user::UserImageCreateRequest,
};
use shared::domain::media::WebMediaUrlCreateRequest;
use shared::domain::meta::ImageTagIndex;
use shared::domain::search::WebImageSearchQuery;
use shared::media::MediaKind;
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
use std::rc::Rc;
use url::Url;
use utils::prelude::*;
use web_sys::File;

pub async fn web_to_image(url: Url) -> Result<Image, ()> {
    let req = WebMediaUrlCreateRequest { url };

    let res = endpoints::media::Create::api_with_auth(Some(req))
        .await
        .map_err(|_| ())?;

    if !matches!(res.kind, MediaKind::Image(_)) {
        unreachable!("Only images here");
    }

    wait_for_upload_ready(&res.id, MediaLibrary::Web, None).await;

    Ok(Image {
        id: ImageId(res.id),
        lib: MediaLibrary::Web,
    })
}

impl State {
    pub fn set_selected(&self, image: Image) {
        if let Some(on_select) = self.callbacks.on_select.as_ref() {
            on_select(image.clone());
        }
        add_recent(self, &image);
    }
}

pub fn on_web_image_click(state: Rc<State>, url: Url) {
    state.loader.load(clone!(state => async move {

        let image = web_to_image(url).await.expect_ji("Couldn't upload image");
        state.set_selected(image);

    }));
}

pub async fn get_styles() -> Vec<ImageStyle> {
    let res = api_with_auth::<MetadataResponse, (), ()>(
        endpoints::meta::Get::PATH,
        endpoints::meta::Get::METHOD,
        None,
    )
    .await;
    res.unwrap_ji().image_styles
}

pub fn search(state: Rc<State>, page: Option<u32>) {
    state.loader.load(clone!(state => async move {
        match state.search_mode.get_cloned() {
            SearchMode::Sticker(_) => search_async(Rc::clone(&state), page.unwrap_or_default()).await,
            SearchMode::Web(_) => search_async_web(Rc::clone(&state)).await,
        };
    }));
}

pub fn fetch_init_data(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let search = search_async(Rc::clone(&state), 0);
        if state.recent {
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
    // let kind = match &state.options.checkbox_kind {
    //     Some(ImageSearchKind::StickersFilter) if state.checkbox_checked.get() => Some(ImageKind::Sticker),
    //     _ => None,
    // };

    let mut tags = state.options.tags.clone().unwrap_or_default();
    match &state.options.kind {
        ImageSearchKind::Background | ImageSearchKind::Overlay => {
            let tag = match &state.options.kind {
                ImageSearchKind::Background => ImageTag::BackgroundLayer1,
                ImageSearchKind::Overlay => ImageTag::BackgroundLayer2,
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

    let req = WebImageSearchQuery {
        q: state.query.lock_ref().clone(),
        ..Default::default()
    };

    let res = endpoints::search::WebImageSearch::api_with_auth(Some(req)).await;

    match res {
        Ok(res) => {
            state
                .search_mode
                .set(SearchMode::Web(Rc::new(MutableVec::new_with_values(
                    res.images,
                ))));
        }
        Err(e) => {
            log::error!("{:#?}", e);
        }
    }
}

async fn search_async(state: Rc<State>, page: u32) {
    if state.user.borrow().is_none() {
        get_user(Rc::clone(&state)).await;
    }

    let affiliations = match &*state.user.borrow() {
        Some(user) => user.affiliations.clone(),
        None => unreachable!("User should exist"),
    };

    let kind = match &state.options.kind {
        ImageSearchKind::Sticker if state.checkbox_checked.get() => Some(ImageKind::Sticker),
        _ => None,
    };

    let mut tags = state.options.tags.clone().unwrap_or_default();
    match &state.options.kind {
        ImageSearchKind::Background | ImageSearchKind::Overlay => {
            let tag = match &state.options.kind {
                ImageSearchKind::Background => ImageTag::BackgroundLayer1,
                ImageSearchKind::Overlay => ImageTag::BackgroundLayer2,
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
        page: Some(page),
        styles: state.selected_styles.borrow().iter().copied().collect(),
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
            let images: Vec<Image> = res
                .images
                .iter()
                .map(|i| Image {
                    id: i.metadata.id,
                    lib: MediaLibrary::Global,
                })
                .collect();

            // if it's the first page replace otherwise append
            if page == 0 {
                state
                    .search_mode
                    .set(SearchMode::Sticker(Rc::new(MutableVec::new_with_values(
                        images,
                    ))));
            } else {
                // adding each item manually, wonder if there's a more efficient way
                let search_mode = state.search_mode.lock_mut();

                match &*search_mode {
                    SearchMode::Web(_) => {
                        unreachable!("Should not get non 0 page for sticker when in web mode")
                    }
                    SearchMode::Sticker(images_on_page) => {
                        let mut images_on_page = images_on_page.lock_mut();
                        images.into_iter().for_each(|image| {
                            images_on_page.push_cloned(image);
                        });
                    }
                }
            }

            let next_page = page + 1;
            if next_page >= res.pages {
                *state.next_page.borrow_mut() = NextPage::End;
            } else {
                *state.next_page.borrow_mut() = NextPage::Page(next_page);
            }
        }
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
        id: image.id,
        library: image.lib,
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
