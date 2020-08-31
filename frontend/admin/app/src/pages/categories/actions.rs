use shared::domain::category::*;
use std::convert::TryInto;
use core::{
    routes::{Route, UserRoute},
    fetch::{
        FetchResult,
        admin::category as fetch_category,
    },
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



pub async fn load_categories(page:Rc<CategoriesPage>) {

    fn load_children(categories:Vec<Category>, parent:Rc<MutableCategory>) {
        for cat in categories.into_iter() {
            let item = MutableCategory::append_child(Some(cat.name.to_string()), cat.id.0.to_string(), Some(parent.clone()));
            if !cat.children.is_empty() {
                load_children(cat.children, item);
            }
        }
    }

    log::info!("it should try to load...");
    let resp = fetch_category::get_all().await;
    match resp {
        Ok(resp) => {
            let mut categories:Vec<Category> = resp.categories;
            let mut parent:Rc<MutableCategory> = page.categories_root.clone();
            
            load_children(categories, parent);

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


pub fn create_category(parent:Rc<MutableCategory>) {
    spawn_local(async move {
        let parent_id = {
            if parent.parent.is_none() {
                None
            } else {
                Some(parent.id.as_ref())
            }
        };
        let resp = fetch_category::create(super::data::EMPTY_NAME.to_string(), parent_id).await;

        match resp {
            Ok(resp) => {
                let _ = MutableCategory::append_child(None, resp.id.0.to_string(), Some(parent.clone()));
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
    })
}


