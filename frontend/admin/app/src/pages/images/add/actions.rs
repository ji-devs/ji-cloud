use shared::{
    api::endpoints::{ApiEndpoint, image::*},
    domain::image::*,
    error::image::*
};
use core::{
    path::api_url,
    fetch::{api_with_auth, api_with_auth_empty, FetchResult, upload_file}
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use url::Url;
use web_sys::File;

pub async fn create_image(file:File) -> Result<String, ()> {
    match _create_image_api().await {
        Err(_) => { return Err(()) },
        Ok(res) => {
            let CreateResponse { id, upload_url} = res;
            let id = id.0.to_string();
            log::info!("got id: {}", id);

            upload_file(&upload_url.to_string(), &file).await
                .map_err(|_| ())
                .map(|_| id)
        }
    }
}


async fn _create_image_api() -> FetchResult < <Create as ApiEndpoint>::Res, <Create as ApiEndpoint>::Err> {
    let req:<Create as ApiEndpoint>::Req = CreateRequest {
        name: "".to_string(),
        description: "".to_string(),
        is_premium: false,
        publish_at: None,
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new()
    };

    api_with_auth(&api_url(Create::PATH), Create::METHOD, Some(req)).await
}
/*

    //needs to be a function due to orphan rule
    fn category_id_from_str(id:&str) -> CategoryId {
        CategoryId(uuid_from_str(id))
    }
    //needs to be a function due to orphan rule
    fn uuid_from_str(id:&str) -> Uuid {
        Uuid::parse_str(id).unwrap_throw()
    }

    pub async fn get_all() -> FetchResult < <Get as ApiEndpoint>::Res, <Get as ApiEndpoint>::Err> {
        let req:<Get as ApiEndpoint>::Req = GetCategoryRequest {
            ids: Vec::new(), 
            scope: Some(CategoryTreeScope::Decendants)
        };
        
        let query = serde_qs::to_string(&req).unwrap_throw();

        let path = api_url(&format!("{}?{}", Get::PATH, query)); 

        api_with_auth::<_,_,()>(&path, Get::METHOD, None).await
    }
    */
