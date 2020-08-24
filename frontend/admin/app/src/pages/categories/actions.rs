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
use super::CategoriesPage;
use futures::future::ready;


pub struct MutableCategory {
    pub id: String,
    pub name: Mutable<String>,
    pub children: MutableVec<Rc<MutableCategory>>,
    pub parent: Option<Rc<MutableCategory>>,
}

impl MutableCategory {
    pub fn append_child(name:String, id: String, parent: Option<Rc<MutableCategory>>) -> Rc<Self> {
        let parent_clone = parent.clone();
        let _self = Rc::new(Self {
            id,
            name: Mutable::new(name),
            children: MutableVec::new(),
            parent
        });

        if let Some(parent) = parent_clone {
            parent.children.lock_mut().push_cloned(_self.clone());
        }

        _self
    }

    pub fn parent_index(&self) -> Option<usize> {
        self.parent.as_ref().map(|parent| {
            parent.children.lock_ref().as_slice().iter().position(|x| x.id == self.id)
        })
        .flatten()
    }
    pub fn parent_len(&self) -> Option<usize> {
        self.parent.as_ref().map(|parent| {
            parent.children.lock_ref().as_slice().iter().len()
        })
    }

    pub fn delete(&self) {
        self.parent.as_ref().unwrap_throw().children.lock_mut().retain(|cat| cat.id != self.id);
    }

    pub fn move_up(&self) -> Option<(usize, usize)> {
        let current_index = self.parent_index().unwrap_throw();
        if current_index > 0 {
            let info = (current_index, current_index -1);
            self.parent.as_ref().unwrap_throw().children.lock_mut().move_from_to(info.0, info.1);

            Some(info)
        } else {
            None
        }
    }
    pub fn move_down(&self) -> Option<(usize, usize)> {
        let current_index = self.parent_index().unwrap_throw();
        let parent_len = self.parent_len().unwrap_throw();
        if current_index < parent_len - 1 {
            let info = (current_index, current_index + 1);
            self.parent.as_ref().unwrap_throw().children.lock_mut().move_from_to(info.0, info.1);
            Some(info)
        } else {
            None
        }
    }
}

pub async fn load_categories(page:Rc<CategoriesPage>) {

    fn load_children(categories:Vec<Category>, parent:Rc<MutableCategory>) {
        for cat in categories.into_iter() {
            let item = MutableCategory::append_child(cat.name.to_string(), cat.id.0.to_string(), Some(parent.clone()));
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


pub fn create_category(parent:Rc<MutableCategory>, name:String) {
    spawn_local(async move {
        let parent_id = {
            if parent.parent.is_none() {
                None
            } else {
                Some(parent.id.as_ref())
            }
        };
        let resp = fetch_category::create(name.clone(), parent_id).await;

        match resp {
            Ok(resp) => {
                let _ = MutableCategory::append_child(name, resp.id.0.to_string(), Some(parent.clone()));
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

pub fn delete_category(cat:&MutableCategory) {
    cat.delete();
    let id = cat.id.clone();


    spawn_local(
        async move {
            fetch_category::delete(&id).await;
            ()
        }
    )
}


pub fn move_up(cat:&MutableCategory) {
    if let Some((before, after)) = cat.move_up() {
        let id = cat.id.clone();

        /*
        spawn_local(
            async move {
                fetch_category::move_before_sibling(&id, before.try_into().unwrap()).await;
                ()
            }
        )
        */
    }
}
pub fn move_down(cat:&MutableCategory) {
    if let Some((before, after)) = cat.move_down() {
        let id = cat.id.clone();
        let parent_id = cat.parent.as_ref().unwrap_throw().id.clone();
        let parent_len = cat.parent_len().unwrap_throw();

        /*
        spawn_local(
            async move {
                if after == parent_len-1 {
                    fetch_category::move_end(&id, &parent_id).await;
                } else {
                    fetch_category::move_before_sibling(&id, (after+1).try_into().unwrap()).await;
                }
                ()
            }
        )
        */
    }
}


pub fn rename_category(cat:&MutableCategory, name:&str) {
    cat.name.set(name.to_string());
}
