use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlInputElement};
use dominator::{Dom, html, events, clone};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use super::actions;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};
use shared::user::UserProfile;
use shared::category::Category;
use super::actions::*;

pub struct CategoriesPage {
    pub refs: RefCell<Option<CategoryPageRefs>>,
    pub loader_status: Mutable<Option<Result<(), ()>>>,
    pub loader: AsyncLoader,
    pub categories: RefCell<Option<MutableVec<MutableCategory>>>
}

impl Drop for CategoriesPage {
    fn drop(&mut self) {
        log::info!("cleaned up categories page!");
        //self.signin_loader.cancel();
    }
}

impl CategoriesPage {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            refs: RefCell::new(None),
            loader_status: Mutable::new(None),
            loader: AsyncLoader::new(),
            categories: RefCell::new(None),
        });

        _self.loader.load(load_categories(_self.clone()));

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::categories(), {
            .with_data_id!("categories", {
                .child_signal(_self.loader_status.signal_ref(clone!(_self => move |status| {
                    Some(status
                        .as_ref()
                        .map(|status| match status {
                            Ok(_) => {
                                html!("div", {
                                    .children_signal_vec({
                                        let categories = _self.categories.borrow();
                                        categories.as_ref().unwrap_throw().signal_vec_cloned().map(|category| {
                                            MutableCategoryDom::render(MutableCategoryDom::new(category))
                                        })
                                    })
                                })
                            },
                            Err(_) => html!("div", {.text("error!")})
                        })
                        .unwrap_or(html!("div", {.text("loading...")}))
                    )
                })))
            })
            .with_data_id!("cat-create", {
                .event(clone!(_self => move |_evt:events::Click| {
                    spawn_local(
                        create_category(_self.clone(), _self.clone().refs.borrow().as_ref().unwrap_throw().new_name.value())
                        );
                }))
            })
            .after_inserted(clone!(_self => move |elem| {
                _self.stash_refs(elem)
            }))
        })
    }

    fn stash_refs(&self, parent:HtmlElement) {
        *self.refs.borrow_mut() = Some(CategoryPageRefs::new(&parent));
    }

}

pub struct CategoryPageRefs {
    new_name: HtmlInputElement,
}

impl CategoryPageRefs {
    pub fn new(parent:&HtmlElement) -> Self {
        Self {
            new_name: parent.select(&data_id("cat-new-name")),
        }
    }

}

pub struct MutableCategoryDom {
    category:MutableCategory
}

impl MutableCategoryDom {
    pub fn new(category:MutableCategory) -> Rc<Self> {
        let _self = Rc::new(Self { category });
        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", { 
            .text(&_self.category.name)
            .children_signal_vec(_self.category.children.signal_vec_cloned().map(|category| {
                MutableCategoryDom::render(MutableCategoryDom::new(category))
            }))
        })
    }

}
