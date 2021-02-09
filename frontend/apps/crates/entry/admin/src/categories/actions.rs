use super::state::*;
use std::rc::Rc;
use shared::{
    api::endpoints::{ApiEndpoint,self},
    domain::category::{GetCategoryRequest, CreateCategoryRequest,NewCategoryResponse,CategoryResponse, CategoryTreeScope, CategoryId},
    error::EmptyError
};
use utils::fetch::api_with_auth;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use dominator::clone;

pub fn handle_double_click(cat: &Rc<Category>) {
    if !cat.editing.get() {
        cat.editing.set(true);
    }
}

pub fn toggle_expand_all(cat: &Rc<Category>, flag: bool) {
    cat.expanded.set(flag);

    for child in cat.children.lock_ref().iter() {
        toggle_expand_all(child, flag);
    }
}
pub fn load_categories(state:Rc<State>) {
    state.loader.load(clone!(state => async move {
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
    }));
}


pub fn add_category(state:Rc<State>, parent: Option<Rc<Category>>) {
    state.loader.load(clone!(state => async move {
        let name = crate::strings::STR_NEW_CATEGORY_NAME.to_string();

        let req = CreateCategoryRequest {
            name: name.clone(),
            parent_id: parent.map(|cat| cat.id)
        };

        match api_with_auth::<NewCategoryResponse, EmptyError, _>(endpoints::category::Create::PATH, endpoints::category::Create::METHOD, Some(req)).await {
            Ok(resp) => {
                let cat = Rc::new(Category::new(resp.id, name));
                state.categories.lock_mut().push_cloned(cat);
            },
            Err(_) => {
                log::info!("err!")
            }
        }
    }));
}
