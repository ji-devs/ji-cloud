use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::image::{
        ImageBrowsePath, ImageBrowseQuery, ImageGetPath, ImageId, ImageResponse, ImageSearchPath,
        ImageSearchQuery,
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use super::{editable_image::EditableImage, state::ImageTable, FetchMode};

impl ImageTable {
    pub fn search_images(self: &Rc<Self>, query: String) {
        let state = self;

        state.clear_selected();

        let mut fetch_mode = state.fetch_mode.borrow_mut();
        if query.is_empty() {
            *fetch_mode = FetchMode::Browse;
        } else {
            *fetch_mode = FetchMode::Search(query);
        }

        state.active_page.set(0);

        state.loader.load(clone!(state => async move {
            state.load_images().await;
        }));
    }

    pub fn checkbox_change(&self, image_id: &ImageId, shift_key: bool) {
        let mut selected_images = self.selected_images.lock_mut();
        if selected_images.contains(&image_id) {
            selected_images.remove(&image_id);
        } else {
            selected_images.insert(*image_id);
        }

        if shift_key && !selected_images.is_empty() {
            // drop, next function needs the lock
            drop(selected_images);
            self.handle_between_checked(&image_id);
        }
    }

    fn handle_between_checked(&self, current_id: &ImageId) {
        let images = self.images.lock_ref();
        let mut selected_images = self.selected_images.lock_mut();

        let current_pos = images.iter().position(|i| &i.id == current_id).unwrap_ji();

        let current_is_selected = selected_images.contains(&current_id);

        let find_pos_func = |i: &Rc<EditableImage>| {
            if current_is_selected {
                selected_images.contains(&i.id)
            } else {
                !selected_images.contains(&i.id)
            }
        };

        let between;
        let previous = images[..current_pos].iter().rposition(find_pos_func);
        if let Some(previous) = previous {
            // log::info!("previous: {previous}");
            between = &images[previous + 1..current_pos];
        } else {
            let next = images
                .iter()
                .enumerate()
                .position(|(i, image)| i > current_pos && find_pos_func(image));
            if let Some(next) = next {
                between = &images[current_pos + 1..next];
            } else {
                between = &images[0..0]
            }
        }

        for image in between {
            if current_is_selected {
                selected_images.insert(image.id);
            } else {
                selected_images.remove(&image.id);
            }
        }
    }

    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.load_images().await;
        }));
    }

    pub async fn load_images(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_images_browse().await,
            FetchMode::Search(query) => self.load_images_search(query.clone()).await,
        };

        self.images.lock_mut().replace_cloned(
            res.images
                .into_iter()
                .map(|meta| Rc::new(meta.metadata.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_images_browse(&self) -> ImageListResponse {
        let req = ImageBrowseQuery {
            page: Some(self.active_page.get()),
            // is_published: Option<bool>,
            ..Default::default()
        };

        match endpoints::image::Browse::api_with_auth(ImageBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => ImageListResponse {
                images: res.images,
                total_pages: res.pages,
            },
        }
    }

    async fn load_images_search(&self, query: String) -> ImageListResponse {
        let req = ImageSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::image::Search::api_with_auth(ImageSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => ImageListResponse {
                images: res.images,
                total_pages: res.pages,
            },
        }
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_images().await;
        }));
    }

    pub async fn get_image(self: Rc<Self>, image_id: ImageId) -> Rc<EditableImage> {
        let image = self
            .images
            .lock_ref()
            .iter()
            .find(|jig| jig.id == image_id)
            .cloned();
        match image {
            Some(image) => image,
            None => Rc::new(self.load_image(&image_id).await),
        }
    }

    async fn load_image(self: &Rc<Self>, image_id: &ImageId) -> EditableImage {
        match endpoints::image::Get::api_with_auth(ImageGetPath(*image_id), None).await {
            Ok(image) => image.into(),
            Err(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ImageListResponse {
    images: Vec<ImageResponse>,
    total_pages: u32,
}
