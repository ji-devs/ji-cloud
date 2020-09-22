use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::utils::templates;
use awsm_web::dom::*;
use super::actions;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use core::routes::{Route, UserRoute};
use shared::domain::{
    user::UserProfile,
    category::Category
};
use super::{data::*, actions::*};

pub struct CategoriesPage {
    pub loader_status: Mutable<Option<Result<(), ()>>>,
    pub loader: AsyncLoader,
    pub categories_root: Rc<MutableCategory>, 
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
            loader_status: Mutable::new(None),
            loader: AsyncLoader::new(),
            categories_root: MutableCategory::append_child(Some("root".to_string()), "-1".to_string(), None) 
        });

        _self.loader.load(load_categories_page(_self.clone()));

        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.loader_status.signal_ref(clone!(_self => move |status| 
                Some(
                    match status {
                        None => Self::render_loading(),
                        Some(res) => match res {
                            Ok(_) => Self::render_loaded(_self.clone()),
                            Err(_) => Self::render_load_failed() 
                        }
                    }
                )
            )))
        })
    }

    pub fn render_loading() -> Dom {
        html!("div", {.text("loading")})
    }
    pub fn render_load_failed() -> Dom {
        html!("div", {.text("failed!")})
    }

    pub fn render_loaded(_self: Rc<Self>) -> Dom {
        elem!(templates::categories(), {
            .with_data_id!("list", {
                .child({
                    MutableCategoryDom::render(Rc::new(RefCell::new(TreeState::new())), MutableCategoryDom::new(_self.categories_root.clone()), _self.clone())
                })
            })
            .with_data_id!("new-cat-btn", {
                .event(clone!(_self => move |_evt:events::Click| {
                    create_category(_self.categories_root.clone());
                }))
            })
        })
    }

}

pub struct TreeState {
    pub current_menu: Option<Rc<Mutable<bool>>>
}
impl TreeState {
    pub fn new() -> Self {
        Self { current_menu: None }
    }
    pub fn close_menu(&mut self) {
        if let Some(menu_visible) = &self.current_menu {
            menu_visible.set(false);
        }

        self.current_menu = None; 
    }
    pub fn open_menu(&mut self, menu:Rc<Mutable<bool>>) {
        self.close_menu();

        menu.set(true);
        self.current_menu = Some(menu); 
    }
}

pub struct MutableCategoryDom {
    category:Rc<MutableCategory>,
    menu_visible: Rc<Mutable<bool>>,
    selected: Mutable<bool>,
    editing_mode: Mutable<bool>,
    latest_text_input: RefCell<Option<HtmlInputElement>>
}

impl MutableCategoryDom {
    pub fn new(category:Rc<MutableCategory>) -> Rc<Self> {
        let editing_mode = category.name.lock_ref().is_none();

        let _self = Rc::new(Self { 
            category ,
            menu_visible: Rc::new(Mutable::new(false)),
            selected: Mutable::new(false),
            editing_mode: Mutable::new(editing_mode),
            latest_text_input: RefCell::new(None),
        });
        _self
    }
    
    pub fn render(_tree: Rc<RefCell<TreeState>>, _self: Rc<Self>, _page:Rc<CategoriesPage>) -> Dom {

        fn init_elem(_tree: Rc<RefCell<TreeState>>, _self:Rc<MutableCategoryDom>, _page:Rc<CategoriesPage>, dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
            apply_methods!(dom, {

                .with_data_id!("children", {
                    .children_signal_vec(_self.category.children.signal_vec_cloned().map(
                        clone!(_page, _self, _tree => move |category| {
                            MutableCategoryDom::render(_tree.clone(), MutableCategoryDom::new(category), _page.clone())
                        })
                    ))
                })
                .with_data_id!("label", {
                    .child_signal(_self.editing_mode.signal().map(clone!(_self => move |editing| {

                        Some(if editing {
                            elem!(templates::category_label_input(), {
                                .property("value", {
                                    match &*_self.category.name.lock_ref() {
                                        Some(name) => name.clone(),
                                        None => super::data::EMPTY_NAME.to_string()
                                    }
                                }) 
                                .event(clone!(_self => move |evt:events::KeyDown| {
                                    if evt.key() == "Enter" {
                                        if let Some(target) = evt.target() {
                                            let input:HtmlInputElement = target.unchecked_into();
                                            let value = input.value();
                                            _self.editing_mode.set(false);
                                            _self.category.rename(value);
                                        }
                                    } else if evt.key() == "Escape" {
                                        _self.editing_mode.set(false);
                                    }
                                }))
                                .global_event(clone!(_self => move |evt:events::MouseDown| {
                                    if let Some(target) = evt.target() {
                                        let element:Element = target.unchecked_into();
                                        if !element.closest_data_id("input").is_some() {
                                            //can't use target since it's specifically NOT the
                                            //input here (click outside)
                                            //that's why we needed to stash the after_inserted
                                            if let Some(input) = &*_self.latest_text_input.borrow() {
                                                let value = input.value();
                                                _self.category.rename(value);
                                            }
                                            _self.editing_mode.set(false);
                                        }
                                    }
                                }))
                                .focused(true)
                                .after_inserted(clone!(_self => move |elem| {
                                    *_self.latest_text_input.borrow_mut() = Some(elem.unchecked_into());
                                }))
                            })
                        } else {
                            let name_signal = _self.category.name.signal_ref(move |name| {
                                let name = match name {
                                    Some(name) => name.clone(),
                                    None => super::data::EMPTY_NAME.to_string()
                                };

                                name
                            });
                            elem!(templates::category_label_display(), { 
                                .text_signal(name_signal) 

                                .event(clone!(_self => move |_evt:events::DoubleClick| {
                                    _self.editing_mode.set(true);
                                }))
                            })
                        })
                    })))
                })
                .with_data_id!("menu-container", {
                    .child_signal(_self.menu_visible.signal().map(clone!(_self, _tree => move |menu_visible| {
                        if !menu_visible {
                            None
                        } else { 
                            Some(
                                elem!(templates::category_menu(), {
                                    .with_data_id!("close", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _self.editing_mode.set(false);
                                            _tree.borrow_mut().close_menu();
                                        }))
                                    })
                                    .with_data_id!("add", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _tree.borrow_mut().close_menu();

                                            _self.selected.set(true);
                                            create_category(_self.category.clone());
                                        }))
                                    })
                                    .with_data_id!("delete", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _tree.borrow_mut().close_menu();
                                            _self.category.delete();
                                        }))
                                    })
                                    .with_data_id!("move-up", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _tree.borrow_mut().close_menu();
                                            _self.category.move_up();
                                        }))
                                    })
                                    .with_data_id!("move-down", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _tree.borrow_mut().close_menu();
                                            _self.category.move_down();
                                        }))
                                    })
                                    .with_data_id!("rename", {
                                        .event(clone!(_self, _tree => move |_evt:events::Click| {
                                            _tree.borrow_mut().close_menu();
                                            _self.editing_mode.set(true);
                                        }))
                                    })
                                    .global_event(clone!(_self, _tree => move |evt:events::Click| {
                                        if let Some(target) = evt.target() {
                                            let element:Element = target.unchecked_into();
                                            if !element.closest_data_id("menu-container").is_some() {
                                                _tree.borrow_mut().close_menu();
                                            }
                                        }
                                    }))
                                })
                            )
                        }
                    })))
                })
                .with_data_id!("menu-toggle-btn", {
                    .event(clone!(_self, _tree => move |_evt:events::Click| {
                        _tree.borrow_mut().open_menu(_self.menu_visible.clone());
                    }))
                })
            })
        }

        if let Some(parent) = &_self.category.parent {
            let is_main_tree = parent.parent.is_none();
           
            if is_main_tree {
                html!("div", {
                    .child_signal(_self.selected.signal().map(clone!(_self, _page => move |selected| {
                            let builder = init_elem(_tree.clone(), _self.clone(), _page.clone(), DomBuilder::new(templates::category_main(&_self.category.id, selected)));
                            let builder = apply_methods!(builder, {
                                .with_data_id!("arrow", {
                                    .event(clone!(_self => move |_evt:events::Click| {
                                        _self.selected.set(!selected);
                                    }))
                                })
                            });

                            Some(builder.into_dom())
                        }))
                    )
                })
            } else {
                html!("div", {
                    .child_signal(_self.selected.signal().map(clone!(_self, _page => move |selected| {
                        let builder = init_elem(_tree.clone(), _self.clone(), _page.clone(), DomBuilder::new(templates::category_sub(&_self.category.id)));
                        Some(builder.into_dom())
                    })))
                })
            }
        } else {
            html!("div", {
                .children_signal_vec(_self.category.children.signal_vec_cloned().map(
                    clone!(_page, _self, _tree => move |category| {
                        MutableCategoryDom::render(_tree.clone(), MutableCategoryDom::new(category), _page.clone())
                    })
                ))
            })
        }
    }

}
