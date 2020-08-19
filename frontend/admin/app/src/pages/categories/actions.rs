use shared::category::*;
use core::{
    routes::{Route, UserRoute},
    fetch::{
        FetchResult,
        admin::category as fetch_category,
    },
    storage,
};
use futures_signals::signal_vec::MutableVec;
use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::clone;
use std::rc::Rc;
use super::CategoriesPage;
use futures::future::ready;

#[derive(Clone)]
pub struct MutableCategory {
    pub id: String,
    pub name: String,
    pub children: Rc<MutableVec<MutableCategory>>,
}

impl MutableCategory {
    pub fn new(cat:Category) -> Self {
        Self {
            id: cat.id.0.to_string(),
            name: cat.name,
            children: Rc::new(convert_categories(cat.children))
        }
    }
    pub fn new_direct(name:String, id: CategoryId) -> Self {
        Self {
            id: id.0.to_string(),
            name,
            children: Rc::new(MutableVec::new())
        }
    }
    pub fn new_temp(name:String) -> Self {
        Self {
            id: name.clone(),
            name,
            children: Rc::new(MutableVec::new())
        }
    }
}

fn convert_categories(cats:Vec<Category>) -> MutableVec<MutableCategory> {
    MutableVec::new_with_values(cats.into_iter().map(MutableCategory::new).collect())
}
pub async fn load_categories(page:Rc<CategoriesPage>) {

    log::info!("it should try to load...");
    let resp = fetch_category::get_all().await;
    match resp {
        Ok(resp) => {
            log::info!("got categories!");
            *page.categories.borrow_mut() = Some(convert_categories(resp.categories));
            page.loader_status.set(Some(Ok(())));
        }, 
        Err(err) => {
            match err {
                Ok(err) => {
                    log::info!("{}", serde_json::to_string(&err).unwrap());
                },
                Err(err) => {
                    log::info!("{:?}", err); 
                    //log::info!("internal error?");
                }
            }
            page.loader_status.set(Some(Err(())))
        }
    }
}


pub async fn create_category(page:Rc<CategoriesPage>, name:String) {
    let resp = fetch_category::create(name.clone(), None).await;

    match resp {
        Ok(resp) => {

            let mut categories = page.categories.borrow_mut();
            let mut categories = categories.as_mut().unwrap();

            let cat = MutableCategory::new_direct(name, resp.id);
            categories.lock_mut().push_cloned(cat);

        }, 
        Err(err) => {
            match err {
                Ok(err) => {
                    log::info!("{}", serde_json::to_string(&err).unwrap());
                },
                Err(err) => {
                    log::info!("{:?}", err); 
                    //log::info!("internal error?");
                }
            }
        }
    }
}
