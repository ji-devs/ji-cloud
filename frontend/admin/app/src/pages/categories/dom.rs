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
use web_sys::{HtmlElement, HtmlInputElement};
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
use super::actions::*;

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

        _self.loader.load(load_categories(_self.clone()));

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
                    MutableCategoryDom::render(MutableCategoryDom::new(_self.categories_root.clone()), _self.clone())
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

pub struct MutableCategoryDom {
    category:Rc<MutableCategory>,
    menu_visible: Mutable<bool>,
    selected: Mutable<bool>,
    editing_mode: Mutable<bool>,
}

impl MutableCategoryDom {
    pub fn new(category:Rc<MutableCategory>) -> Rc<Self> {
        let editing_mode = category.name.lock_ref().is_none();

        let _self = Rc::new(Self { 
            category ,
            menu_visible: Mutable::new(false),
            selected: Mutable::new(false),
            editing_mode: Mutable::new(editing_mode),
        });
        _self
    }
    
    pub fn render(_self: Rc<Self>, _page:Rc<CategoriesPage>) -> Dom {

        fn init_elem(_self:Rc<MutableCategoryDom>, _page:Rc<CategoriesPage>, dom:DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
            apply_methods!(dom, {

                .with_data_id!("children", {
                    .children_signal_vec(_self.category.children.signal_vec_cloned().map(
                        clone!(_page, _self => move |category| {
                            MutableCategoryDom::render(MutableCategoryDom::new(category), _page.clone())
                        })
                    ))
                })
                .with_data_id!("label", {
                    .child_signal(_self.editing_mode.signal().map(clone!(_self => move |editing| {
                        let name_signal = _self.category.name.signal_ref(|name| {
                            match name {
                                Some(name) => name.clone(),
                                None => super::data::EMPTY_NAME.to_string()
                            }
                        });

                        Some(if editing {
                            elem!(templates::category_label_input(), {
                                .property_signal("value", name_signal) 
                                    /*
                                .event(clone!(_self => move |evt:events::Input| {
                                    if let Some(value) = evt.value() {
                                        _self.category.name.set(Some(value));
                                    }
                                }))
                                */
                                .event(clone!(_self => move |evt:events::KeyDown| {
                                    if evt.key() == "Enter" {
                                        if let Some(target) = evt.target() {
                                            let input:HtmlInputElement = target.unchecked_into();
                                            let value = input.value();
                                            _self.category.name.set(Some(value.clone()));
                                            _self.editing_mode.set(false);
                                            rename_category(&_self.category, value);
                                        }
                                    } else if evt.key() == "Escape" {
                                        _self.editing_mode.set(false);
                                    }
                                }))
                                .focused(true)
                            })
                        } else {
                            elem!(templates::category_label_display(), { .text_signal(name_signal) })
                        })
                    })))
                })
                .with_data_id!("menu", {
                    .class_signal("hidden", _self.menu_visible.signal_ref(|x| !*x))
                    .with_data_id!("close", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.editing_mode.set(false);
                            _self.menu_visible.set(false);
                        }))
                    })
                    .with_data_id!("add", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.menu_visible.set(false);
                            create_category(_self.category.clone());
                        }))
                    })
                    .with_data_id!("delete", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.menu_visible.set(false);
                            delete_category(&_self.category);
                        }))
                    })
                    .with_data_id!("move-up", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.menu_visible.set(false);
                            move_up(&_self.category);
                        }))
                    })
                    .with_data_id!("move-down", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.menu_visible.set(false);
                            move_down(&_self.category);
                        }))
                    })
                    .with_data_id!("rename", {
                        .event(clone!(_self => move |_evt:events::Click| {
                            _self.editing_mode.set(true);
                            _self.menu_visible.set(false);
                        }))
                    })
                    .global_event(clone!(_self => move |_evt:events::Click| {
                        //TODO close if clicked outside menu
                    }))
                })
                .with_data_id!("menu-toggle-btn", {
                    .event(clone!(_self => move |_evt:events::Click| {
                        log::info!("MENY TOGGLED!");
                        _self.menu_visible.replace_with(|x| !*x);
                    }))
                })
            })
        }

        if let Some(parent) = &_self.category.parent {
            let is_main_tree = parent.parent.is_none();
           
            if is_main_tree {
                html!("div", {
                    .child_signal(_self.selected.signal().map(clone!(_self, _page => move |selected| {
                            let builder = init_elem(_self.clone(), _page.clone(), DomBuilder::new(templates::category_main(&_self.category.id, selected)));
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
                        let builder = init_elem(_self.clone(), _page.clone(), DomBuilder::new(templates::category_sub(&_self.category.id)));
                        Some(builder.into_dom())
                    })))
                })
            }
        } else {
            html!("div", {
                .children_signal_vec(_self.category.children.signal_vec_cloned().map(
                    clone!(_page, _self => move |category| {
                        MutableCategoryDom::render(MutableCategoryDom::new(category), _page.clone())
                    })
                ))
            })
        }
    }

}
