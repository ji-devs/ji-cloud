use shared::{
    api::{ApiEndpoint, endpoints},
    domain::{Publish, image::*, category::*, meta::*},
    error::{EmptyError, MetadataNotFound},
};
use utils::{routes::*, fetch::{api_with_auth, api_no_auth, api_upload_file, api_with_auth_empty}};
use dominator::clone;
use super::state::*;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::File;
use super::sections::common::categories::MutableCategory;

pub fn load_initial(state: Rc<State>) -> Rc<RefCell<Option<(Rc<MutableImage>, Rc<Vec<Rc<MutableCategory>>>, Rc<MetadataResponse>)>>> {
    let ret = Rc::new(RefCell::new(None));

    state.loader.load(clone!(state, ret => async move {

        let path = endpoints::image::Get::PATH.replace("{id}",&state.id.0.to_string());
        let cat_req = GetCategoryRequest{ ids: Vec::new(), scope: Some( CategoryTreeScope::Decendants) } ;
        match (
            api_with_auth::<ImageResponse, EmptyError, ()>(&path, endpoints::image::Get::METHOD, None).await,
            api_with_auth::<CategoryResponse, EmptyError, _>(&endpoints::category::Get::PATH, endpoints::category::Get::METHOD, Some(cat_req)).await,
            api_no_auth::<MetadataResponse, (), ()>(&endpoints::meta::Get::PATH, endpoints::meta::Get::METHOD, None).await
        ) {
            (Ok(img_resp), Ok(cat_resp), Ok(meta_resp)) => {

                let image:Rc<MutableImage> = Rc::new(img_resp.metadata.into());

                let categories:Rc<Vec<Rc<MutableCategory>>> = Rc::new(cat_resp
                    .categories
                    .into_iter()
                    .map(|cat| Rc::new(cat.into()))
                    .collect()
                );

                let meta:Rc<MetadataResponse> = Rc::new(meta_resp);

                *ret.borrow_mut() = Some((image, categories, meta));
                state.loaded.set(true);
            },
            errors => {
                log::error!("error loading initial data!")
            }
        }
    }));

   ret 
}

pub fn save(state: Rc<State>, req:ImageUpdateRequest) {
    state.loader.load(clone!(state => async move {
   
        let path = endpoints::image::UpdateMetadata::PATH.replace("{id}",&state.id.0.to_string());
        match api_with_auth_empty::<MetadataNotFound, _>(&path, endpoints::image::UpdateMetadata::METHOD, Some(req)).await {
            Ok(_) => {
            },
            Err(err) => {
                log::error!("couldn't save!");
            },
        }
    }));
}


pub fn on_file(state: Rc<State>, image: Rc<MutableImage>, file: File) {
    state.loader.load(clone!(state => async move {

        let path = endpoints::image::Upload::PATH.replace("{id}",&state.id.0.to_string());
        match api_upload_file(&path, &file, endpoints::image::Upload::METHOD).await {
            Ok(_) => {
                image.id.replace_with(|id| id.clone());
            },
            Err(_) => {
                log::error!("error uploading!");
            }
        }
    }))
}


pub fn toggle_premium(state: Rc<State>, image: Rc<MutableImage>, is_premium:bool) {
    image.is_premium.set_neq(is_premium);
    save(state, ImageUpdateRequest{
        is_premium: Some(is_premium),
        ..ImageUpdateRequest::default()

    });
}

pub fn change_name(state: Rc<State>, image: Rc<MutableImage>, name:String) {
    image.name.set_neq(name.clone());
    save(state, ImageUpdateRequest{
        name: Some(name),
        ..ImageUpdateRequest::default()

    });
}

pub fn change_description(state: Rc<State>, image: Rc<MutableImage>, description:String) {
    image.description.set_neq(description.clone());
    save(state, ImageUpdateRequest{
        description: Some(description),
        ..ImageUpdateRequest::default()

    });
}

pub fn delete(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        let path = endpoints::image::Delete::PATH.replace("{id}",&state.id.0.to_string());
        match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::image::Delete::METHOD, None).await {
            Ok(_) => {
                let route:String = Route::Admin(AdminRoute::ImageSearch(None)).into();
                dominator::routing::go_to_url(&route);
            },
            Err(err) => {
                log::error!("couldn't save!");
            },
        }
    }));
}

pub fn publish(state: Rc<State>) {
    save(state, ImageUpdateRequest{
        publish_at: Some(Some(Publish::now())), 
        ..ImageUpdateRequest::default()

    });
}
