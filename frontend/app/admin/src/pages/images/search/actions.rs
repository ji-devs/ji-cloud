use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*
};
use core::{
    path::api_url,
    fetch::{api_with_auth, api_with_auth_empty, upload_file}
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
            src: resp.url.to_string(),
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

    //TODO - maybe make query / serde_qs part of basic fetch options
    //since this mistake is hard to catch when forgotten
    let query = serde_qs::to_string(&req).unwrap_throw();

    let path = api_url(&format!("{}?{}", Search::PATH, query)); 
    api_with_auth::<_,_,()>(&path, Search::METHOD, None).await
}
