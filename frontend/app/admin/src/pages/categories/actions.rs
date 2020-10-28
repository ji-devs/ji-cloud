use std::convert::TryInto;
use core::{
    routes::{Route, UserRoute},
    fetch::{api_with_auth, api_with_auth_empty},
    path::api_url,
    storage,
};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::MutableVec};
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use dominator::clone;
use std::rc::Rc;
use super::{dom::CategoriesPage, data::*};
use futures::future::ready;
use shared::{
    api::endpoints::{ApiEndpoint, category::*},
    domain::category::*
};
use uuid::Uuid;



pub async fn load_categories_page(page:Rc<CategoriesPage>) {

    fn load_children(categories:Vec<Category>, parent:Rc<MutableCategory>) {
        for cat in categories.into_iter() {
            let item = MutableCategory::append_child(Some(cat.name.to_string()), cat.id.0.to_string(), Some(parent.clone()));
            if !cat.children.is_empty() {
                load_children(cat.children, item);
            }
        }
    }

    let resp = load_categories().await;
    match resp {
        Ok(resp) => {
            let mut categories:Vec<Category> = resp.categories;
            let mut parent:Rc<MutableCategory> = page.categories_root.clone();
            
            load_children(categories, parent);

            page.loader_status.set(Some(Ok(())));
        }, 
        Err(err) => {
            log::info!("{}", serde_json::to_string(&err).unwrap());
            page.loader_status.set(Some(Err(())))
        }
    }
}


pub fn create_category(parent:Rc<MutableCategory>) {
    spawn_local(async move {
        let parent_id = {
            if parent.parent.is_none() {
                None
            } else {
                Some(parent.id.as_ref())
            }
        };
        let resp = _create(super::data::EMPTY_NAME.to_string(), parent_id).await;

        match resp {
            Ok(resp) => {
                let _ = MutableCategory::append_child(None, resp.id.0.to_string(), Some(parent.clone()));
            }, 
            Err(err) => {
                log::info!("{}", serde_json::to_string(&err).unwrap());
            }
        }
    })
}


//needs to be a function due to orphan rule
fn category_id_from_str(id:&str) -> CategoryId {
    CategoryId(uuid_from_str(id))
}
//needs to be a function due to orphan rule
fn uuid_from_str(id:&str) -> Uuid {
    Uuid::parse_str(id).unwrap_throw()
}

pub async fn load_categories() -> Result < <Get as ApiEndpoint>::Res, <Get as ApiEndpoint>::Err> {
    let req:<Get as ApiEndpoint>::Req = GetCategoryRequest {
        ids: Vec::new(), 
        scope: Some(CategoryTreeScope::Decendants)
    };
    
    api_with_auth(&api_url(Get::PATH), Get::METHOD, Some(req)).await
}

async fn _create(name:String, parent_id: Option<&str>) -> Result < <Create as ApiEndpoint>::Res, <Create as ApiEndpoint>::Err> {

    let req:<Create as ApiEndpoint>::Req = CreateCategoryRequest {
        name,
        parent_id: parent_id.map(category_id_from_str)
    };
    api_with_auth(&api_url(Create::PATH), Create::METHOD, Some(req)).await
}

pub async fn rename(id:&str, name:String) -> Result < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
    let path = Update::PATH.replace("{id}",id);
    
    let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
        name: Some(name),
        parent_id: None,
        index: None
    };
    api_with_auth_empty(&api_url(&path), Update::METHOD, Some(req)).await
}

pub async fn move_to(id:&str, index:u16) -> Result < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
    let path = Update::PATH.replace("{id}",id);
    
    let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
        name: None,
        parent_id: None, 
        index: Some(index) 
    };
    api_with_auth_empty(&api_url(&path), Update::METHOD, Some(req)).await
}

pub async fn move_end(id:&str, parent_id:&str) -> Result < <Update as ApiEndpoint>::Res, <Update as ApiEndpoint>::Err> {
    let path = Update::PATH.replace("{id}",id);
    
    let req:<Update as ApiEndpoint>::Req = UpdateCategoryRequest {
        name: None,
        parent_id: Some(Some(category_id_from_str(parent_id))),
        index: None 
    };
    api_with_auth_empty(&api_url(&path), Update::METHOD, Some(req)).await
}

pub async fn delete(id:&str) -> Result < <Delete as ApiEndpoint>::Res, <Delete as ApiEndpoint>::Err> {
    let path = Delete::PATH.replace("{id}",id);

    api_with_auth_empty::<_,()>(&api_url(&path), Delete::METHOD, None).await
}
