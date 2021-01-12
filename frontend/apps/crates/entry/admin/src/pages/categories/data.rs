use shared::domain::category::*;
use std::convert::TryInto;
use utils::{
    routes::{Route, UserRoute},
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
pub const EMPTY_NAME:&'static str = "New Category";
use super::actions;

pub struct MutableCategory {
    pub id: String,
    pub name: Mutable<Option<String>>,
    pub children: MutableVec<Rc<MutableCategory>>,
    pub parent: Option<Rc<MutableCategory>>,
}

impl MutableCategory {
    pub fn append_child(name:Option<String>, id: String, parent: Option<Rc<MutableCategory>>) -> Rc<Self> {
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

    pub fn rename(&self, name:String) {
        let id = self.id.clone();
        self.name.set(Some(name.clone()));

        spawn_local(
            async move {
                actions::rename(&id, name).await;
                ()
            }
        )
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
        let id = self.id.clone();
        spawn_local(
            async move {
                actions::delete(&id).await;
                ()
            }
        )
    }

    pub fn move_up(&self) {
        let current_index = self.parent_index().unwrap_throw();
        if current_index > 0 {
            let target_index = current_index-1; 
            self.parent.as_ref().unwrap_throw().children.lock_mut().move_from_to(current_index, target_index);

            let id = self.id.clone();

            spawn_local(
                async move {
                    actions::move_to(&id, target_index.try_into().unwrap()).await;
                    ()
                }
            )
        } 
    }
    pub fn move_down(&self) {
        let current_index = self.parent_index().unwrap_throw();
        let parent_len = self.parent_len().unwrap_throw();
        if current_index < parent_len - 1 {
            let target_index = current_index + 1;
            self.parent.as_ref().unwrap_throw().children.lock_mut().move_from_to(current_index, target_index);

            let id = self.id.clone();
            let parent_id = self.parent.as_ref().unwrap_throw().id.clone();
            let parent_len = self.parent_len().unwrap_throw();


            spawn_local(
                async move {
                    actions::move_to(&id, target_index.try_into().unwrap()).await;
                    ()
                }
            )
        } 
    }
}
