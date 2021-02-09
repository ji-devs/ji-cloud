use super::state::*;
use std::rc::Rc;
use shared::{
    api::endpoints::{ApiEndpoint,self},
    domain::category::{GetCategoryRequest, CategoryResponse, CategoryTreeScope},
    error::EmptyError
};
use utils::fetch::api_with_auth;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;

pub fn load_categories(state:Rc<State>) {
    
    spawn_local(async move {
        let req = GetCategoryRequest {
            ids: Vec::new(), 
            scope: Some(CategoryTreeScope::Decendants)
        };
        
        match api_with_auth::<CategoryResponse, EmptyError, _>(endpoints::category::Get::PATH, endpoints::category::Get::METHOD, Some(req)).await {
            Ok(resp) => {
                let categories:Vec<Rc<Category>> = resp.categories
                    .into_iter()
                    .map(Category::from)
                    .map(Rc::new)
                    .collect();

                state.categories.lock_mut().replace_cloned(categories);

            },
            Err(_) => {
                log::info!("err!")
            }
        }
        /*
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
        */
    });
}
