use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*,
    media::{MediaLibraryKind, MediaVariant}
};
use utils::{
    fetch::{api_with_auth, api_with_auth_empty, api_upload_file}
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use url::Url;
use web_sys::File;

#[derive(Clone, Debug)]
pub struct BasicImage {
    pub id: String,
    pub src: String,
    pub is_published: bool,
    pub text: String
}
impl BasicImage {
    pub fn new(resp:GetResponse) -> Self {

        let id = resp.metadata.id.0.to_string();
        let src = utils::path::library_image(MediaLibraryKind::Global, MediaVariant::Thumbnail, &id);
        Self {
            id,
            src,
            is_published: resp.metadata.publish_at.is_some(),
            text: resp.metadata.name
        }
    }
}

pub type NumPages = u32;

pub async fn search_images(query:String, page: Option<u32>, is_published: Option<bool>) -> Result<(Vec<BasicImage>, NumPages), ()> {
    _search_images_api(query, page, is_published).await
        .map_err(|err:SearchError| { 
            ()
        })
        .map(|res| {
            let SearchResponse { images, pages } = res;
            let images:Vec<BasicImage> = images
                .into_iter()
                .map(BasicImage::new)
                .collect();

            (images, pages)
        })
}


async fn _search_images_api(query:String, page: Option<u32>, is_published: Option<bool>) -> Result < <Search as ApiEndpoint>::Res, <Search as ApiEndpoint>::Err> {
    let req = SearchQuery {
        q: query,
        page,
        is_published,
        //future query powers :)
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        is_premium: None,
    };

    api_with_auth(Search::PATH, Search::METHOD, Some(req)).await
}
