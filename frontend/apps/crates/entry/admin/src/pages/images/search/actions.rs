use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*
};
use core::{
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
        Self {
            id: resp.metadata.id.0.to_string(),
            src: resp.thumbnail_url.to_string(),
            is_published: resp.metadata.publish_at.is_some(),
            text: resp.metadata.name
        }
    }
}

pub async fn search_images(query:String, page: Option<u32>, is_published: Option<bool>) -> Result<(Vec<BasicImage>, u32), ()> {
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
