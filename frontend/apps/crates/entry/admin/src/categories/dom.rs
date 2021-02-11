use dominator::{Dom, html, clone, with_node};
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use std::rc::Rc;
use std::cell::RefCell;
use super::{state::*, actions};
use web_sys::{HtmlInputElement, HtmlElement};
use utils::{events, routes::*};
use wasm_bindgen::prelude::*;

pub struct CategoriesPage {
}

impl CategoriesPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_categories(state.clone());
        
        html!("empty-fragment", {
            .child(
                html!("category-page", {
                    .children(&mut [
                        html!("button-expand", {
                            .property("slot", "expand")
                            .property("expanded", false)
                            .event(clone!(state => move |evt:events::CustomToggle| {
                                let flag = evt.value();
                                for cat in state.categories.lock_ref().iter() {
                                    actions::toggle_expand_all(&cat, flag);
                                }
                            }))
                        }),
                        html!("category-button-add", {
                            .property("slot", "add")
                            .event(clone!(state => move |evt:events::Click| {
                                actions::add_category(state.clone(), None);
                            }))
                        }),
                        html!("div", {
                            .property("slot", "middle")
                            .children_signal_vec(state.categories.signal_vec_cloned().map(clone!(state => move |category| {
                                CategoryDom::render(category, state.clone(), false) 
                            })))
                        }),

                    ])
                })
            )
            .child(
                html!("window-loader-block", {
                    .property_signal("visible", state.loader.is_loading())
                })
            )

        })

    }
}

pub struct CategoryDom {
}

impl CategoryDom {
    pub fn render(cat: Rc<Category>, state: Rc<State>, is_child: bool) -> Dom {
        html!("dropdown-tree", {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("expanded", cat.expanded.signal())
            .property_signal("hasChildren", cat.has_children_signal())
            .property("isChild", is_child)
            .event(clone!(cat => move |evt:events::ExpandAll| {
                actions::toggle_expand_all(&cat, true)
            }))
            .event(clone!(cat => move |evt:events::CollapseAll| {
                actions::toggle_expand_all(&cat, false)
            }))
            .event(clone!(cat => move |evt:events::Change| {
                log::info!("CHANGE EVENT");
            }))
            .child(ContentDom::render(cat.clone(), state.clone(), is_child))
            .child(html!("div", {
                .property("slot", "children")
                .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state => move |category| {
                    CategoryDom::render(category, state.clone(), true)
                })))
            }))
        })
    }
}

pub struct ContentDom {
}

pub struct ContentState {
    //These are only needed for imperatively toggling via menu
    pub input_ref: RefCell<Option<HtmlElement>>,
    pub menu_ref: RefCell<Option<HtmlElement>>
}

impl ContentState {
    pub fn close_menu(&self) {
        if let Some(menu_ref) = self.menu_ref.borrow().as_ref() {
            unsafe {
                js_sys::Reflect::set(
                    menu_ref, 
                    &JsValue::from_str("visible"), 
                    &JsValue::from_bool(false)
                );
            }
        }
    }
}

impl ContentDom {
    pub fn render(cat: Rc<Category>, state: Rc<State>, is_child: bool) -> Dom {
        let content_state = Rc::new(ContentState {
            input_ref: RefCell::new(None),
            menu_ref: RefCell::new(None),
        });
        

        let visible_signal = Mutable::new(false);
        html!("ellipses-menu-line", {
            .property("slot", "content")
            .children(ContentLineDom::render(cat.clone(), state.clone(), is_child, content_state.clone()))
            .child(MenuDom::render(cat.clone(), state.clone(), content_state.clone()))

            .after_inserted(clone!(content_state => move |elem| {
                *content_state.menu_ref.borrow_mut() = Some(elem);
            }))
        })
    }
}


pub struct ContentLineDom {
}

impl ContentLineDom {
    pub fn render(cat: Rc<Category>, state: Rc<State>, is_child: bool, content_state: Rc<ContentState>) -> Vec<Dom> {

        let mut children:Vec<Dom> = vec![
            html!("input-text-content", {
                .property("slot", "content")
                .property_signal("value", cat.name.signal_cloned())
                .event(clone!(cat, state => move |evt:events::CustomChange| {
                    actions::rename_category(&cat, state.clone(), evt.value());
                }))
                .after_inserted(clone!(content_state => move |elem| {
                    *content_state.input_ref.borrow_mut() = Some(elem);
                }))
            })
        ];

        if !is_child {
            children.push(html!("button-expand", {
                .property("slot", "content")
                .property("expanded", false)
                .event(clone!(cat => move |evt:events::CustomToggle| {
                    actions::toggle_expand_all(&cat, evt.value());
                }))
            }));
        }

        children
        
    }
}

pub struct MenuDom {
}

impl MenuDom {
    pub fn render(cat: Rc<Category>, state: Rc<State>, content_state: Rc<ContentState>) -> Dom {
        html!("div", {
            .property("slot", "menu-content")
            .children(&mut [
                html!("button-text", {
                    .text("add")
                    .event(clone!(state => move |evt:events::Click| {

                    }))
                }),
                html!("button-text", {
                    .text("delete")
                    .event(clone!(state => move |evt:events::Click| {

                    }))
                }),
                html!("button-text", {
                    .text("move up")
                    .event(clone!(state => move |evt:events::Click| {

                    }))
                }),
                html!("button-text", {
                    .text("move down")
                    .event(clone!(state => move |evt:events::Click| {

                    }))
                }),
                html!("button-text", {
                    .text("rename")
                    .event(clone!(content_state => move |evt:events::Click| {
                        if let Some(input_ref) = content_state.input_ref.borrow().as_ref() {
                            unsafe {
                                js_sys::Reflect::set(
                                    input_ref, 
                                    &JsValue::from_str("editing"), 
                                    &JsValue::from_bool(true)
                                );
                            }
                        }

                        content_state.close_menu();

                    }))
                }),
            ])
        })
    }
}
