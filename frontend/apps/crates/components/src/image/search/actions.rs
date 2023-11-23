use crate::firebase::wait_for_upload_ready;
use crate::image::search::state::{ImageSearchKind, NextPage, SearchMode, RECENT_COUNT};
use crate::image::tag::ImageTag;

use super::super::upload::upload_image;
use super::state::{ImageSearch, PremiumableImage};
use dominator::clone;
use futures::future::join;
use futures_signals::signal_vec::MutableVec;
use shared::api::endpoints::image;
use shared::domain::image::recent::{UserRecentImageListPath, UserRecentImageUpsertPath};
use shared::domain::image::user::UserImageCreatePath;
use shared::domain::image::{
    recent::{UserRecentImageListRequest, UserRecentImageUpsertRequest},
    user::UserImageCreateRequest,
};
use shared::domain::image::{ImageId, ImageSearchPath};
use shared::domain::media::{MediaCreatePath, WebMediaUrlCreateRequest};
use shared::domain::meta::{GetMetadataPath, ImageTagIndex};
use shared::domain::search::{WebImageSearchPath, WebImageSearchQuery};
use shared::domain::user::GetProfilePath;
use shared::media::MediaKind;
use shared::{
    api::endpoints,
    domain::{
        image::{CreateResponse, ImageSearchQuery, ImageSize},
        meta::ImageStyle,
        module::body::Image,
    },
    media::MediaLibrary,
};
use std::rc::Rc;
use url::Url;
use utils::prelude::*;
use web_sys::File;

pub async fn web_to_image(url: Url) -> Result<Image, ()> {
    let req = WebMediaUrlCreateRequest { url };

    let res = endpoints::media::Create::api_with_auth(MediaCreatePath(), Some(req))
        .await
        .map_err(|_| ())?;

    if !matches!(res.kind, MediaKind::Image) {
        unreachable!("Only images here but found: {:?}", res.kind);
    }

    wait_for_upload_ready(&res.id, MediaLibrary::Web, None).await;

    Ok(Image {
        id: ImageId(res.id),
        lib: MediaLibrary::Web,
    })
}

impl ImageSearch {
    pub fn set_selected(&self, image: Image) {
        if let Some(on_select) = self.callbacks.on_select.as_ref() {
            on_select(Some(image.clone().into()));
        }
        self.add_recent(&image);
    }

    pub fn clear_selected(&self) {
        if let Some(on_select) = self.callbacks.on_select.as_ref() {
            on_select(None);
        }
    }

    pub fn on_web_image_click(self: &Rc<Self>, url: Url) {
        let state = self;
        state.loader.load(clone!(state => async move {

            let image = web_to_image(url).await.expect_ji("Couldn't upload image");
            state.set_selected(image);

        }));
    }

    pub fn search(self: &Rc<Self>, page: Option<u32>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            match state.search_mode.get_cloned() {
                SearchMode::Sticker(_) => state.search_async(page.unwrap_or_default()).await,
                SearchMode::Web(_) => state.search_async_web().await,
            };
        }));
    }

    pub fn fetch_init_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let search = state.search_async(0);
            if state.recent {
                join(
                    search,
                    state.get_recent()
                ).await;
            } else {
                search.await;
            }
        }));
    }

    async fn search_async_web(self: &Rc<Self>) {
        // let kind = match &state.options.checkbox_kind {
        //     Some(ImageSearchKind::StickersFilter) if state.checkbox_checked.get() => Some(ImageKind::Sticker),
        //     _ => None,
        // };

        let mut tags = self.options.tags.clone().unwrap_or_default();
        match &self.options.kind {
            ImageSearchKind::Background | ImageSearchKind::Overlay => {
                let tag = match &self.options.kind {
                    ImageSearchKind::Background => ImageTag::BackgroundLayer1,
                    ImageSearchKind::Overlay => ImageTag::BackgroundLayer2,
                    _ => unreachable!(),
                };
                if self.checkbox_checked.get() {
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
            q: self.query.lock_ref().clone(),
            image_type: self.selected_image_type.get_cloned(),
        };

        let res =
            endpoints::search::WebImageSearch::api_with_auth(WebImageSearchPath(), Some(req)).await;

        match res {
            Ok(res) => {
                self.search_mode
                    .set(SearchMode::Web(Rc::new(MutableVec::new_with_values(
                        res.images,
                    ))));
            }
            Err(e) => {
                log::error!("{:#?}", e);
            }
        }
    }

    async fn search_async(self: &Rc<Self>, page: u32) {
        if self.user.borrow().is_none() {
            self.get_user().await;
        }

        let affiliations = match &*self.user.borrow() {
            Some(user) => user.affiliations.clone(),
            None => unreachable!("User should exist"),
        };

        let size = match &self.options.kind {
            ImageSearchKind::Sticker if self.checkbox_checked.get() => Some(ImageSize::Sticker),
            _ => None,
        };

        let mut tags = self.options.tags.clone().unwrap_or_default();
        match &self.options.kind {
            ImageSearchKind::Background | ImageSearchKind::Overlay => {
                let tag = match &self.options.kind {
                    ImageSearchKind::Background => ImageTag::BackgroundLayer1,
                    ImageSearchKind::Overlay => ImageTag::BackgroundLayer2,
                    _ => unreachable!(),
                };
                if self.checkbox_checked.get() {
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
            q: self.query.lock_ref().clone(),
            page: Some(page),
            styles: self.selected_styles.borrow().iter().copied().collect(),
            affiliations,
            size,
            tags: tags.iter().map(|x| ImageTagIndex(x.as_index())).collect(),
            tags_priority: self
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

        let res =
            endpoints::image::Search::api_with_auth(ImageSearchPath(), Some(search_query)).await;

        match res {
            Ok(res) => {
                let images: Vec<PremiumableImage> = res
                    .images
                    .iter()
                    .map(|i| PremiumableImage {
                        id: i.metadata.id,
                        lib: MediaLibrary::Global,
                        is_premium: i.metadata.is_premium,
                    })
                    .collect();

                // if it's the first page replace otherwise append
                if page == 0 {
                    self.search_mode.set(SearchMode::Sticker(Rc::new(
                        MutableVec::new_with_values(images),
                    )));
                } else {
                    // adding each item manually, wonder if there's a more efficient way
                    let search_mode = self.search_mode.lock_mut();

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
                    *self.next_page.borrow_mut() = NextPage::End;
                } else {
                    *self.next_page.borrow_mut() = NextPage::Page(next_page);
                }
            }
            Err(e) => {
                log::error!("{:#?}", e);
            }
        }
    }

    async fn get_user(self: &Rc<Self>) {
        match endpoints::user::Profile::api_with_auth(GetProfilePath(), None).await {
            Err(_) => todo!(),
            Ok(user) => {
                *self.user.borrow_mut() = Some(user);
            }
        }
    }

    async fn get_recent(self: &Rc<Self>) {
        let req = UserRecentImageListRequest {
            limit: RECENT_COUNT,
        };

        match image::recent::List::api_with_auth(UserRecentImageListPath(), Some(req)).await {
            Err(_) => log::error!("Error getting recent images"),
            Ok(res) => {
                self.recent_list.lock_mut().replace_cloned(
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

    pub fn add_recent(self: &ImageSearch, image: &Image) {
        {
            let mut recent_list = self.recent_list.lock_mut();

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
        self.loader.load(async {
            let _ = image::recent::Put::api_with_auth(UserRecentImageUpsertPath(), Some(req)).await;
        });
    }

    pub async fn upload_file(self: &Rc<Self>, file: File) {
        let req = UserImageCreateRequest {
            size: ImageSize::Sticker,
        };

        match endpoints::image::user::Create::api_with_auth(UserImageCreatePath(), Some(req)).await
        {
            Ok(resp) => {
                let CreateResponse { id } = resp;

                match upload_image(id, MediaLibrary::User, &file, None).await {
                    Ok(_) => {
                        self.set_selected(Image {
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
}

pub async fn get_styles() -> Vec<ImageStyle> {
    let res = endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await;
    res.unwrap_ji().image_styles
}
