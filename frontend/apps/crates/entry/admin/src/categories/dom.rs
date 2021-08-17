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
                                actions::add_category_root(state.clone());
                            }))
                        }),
                        html!("div", {
                            .property("slot", "middle")
                            .children_signal_vec(state.categories.signal_vec_cloned().map(clone!(state => move |category| {
                                CategoryDom::render(None, category, state.clone()) 
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
    pub fn render(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Dom {
        html!("dropdown-tree", {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("expanded", cat.expanded.signal())
            .property_signal("hasChildren", cat.has_children_signal())
            .property("isChild", parent.is_some())
            .event(clone!(cat => move |evt:events::ExpandAll| {
                actions::toggle_expand_all(&cat, true)
            }))
            .event(clone!(cat => move |evt:events::CollapseAll| {
                actions::toggle_expand_all(&cat, false)
            }))
            .event(clone!(cat => move |evt:events::Change| {
                log::info!("CHANGE EVENT");
            }))
            .child(ContentDom::render(parent, cat.clone(), state.clone()))
            .child(html!("div", {
                .property("slot", "children")
                .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state, cat => move |category| {
                    CategoryDom::render(Some(cat.clone()), category, state.clone())
                })))
            }))
        })
    }
}

pub struct ContentDom {
}


impl ContentDom {
    pub fn render(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Dom {
        let content_state = Rc::new(ContentState::new(parent, cat, state));
        

        let visible_signal = Mutable::new(false);
        html!("menu-ellipses", {
            .property("slot", "content")
            .children(ContentLineDom::render(content_state.clone()))
            .child(MenuDom::render(content_state.clone()))

            .after_inserted(clone!(content_state => move |elem| {
                *content_state.menu_ref.borrow_mut() = Some(elem);
            }))
        })
    }
}


pub struct ContentLineDom {
}

impl ContentLineDom {
    pub fn render(content_state: Rc<ContentState>) -> Vec<Dom> {

        let mut children:Vec<Dom> = vec![
            html!("input-text-content", {
                .property("slot", "content")
                .property_signal("value", content_state.cat.name.signal_cloned())
                .event(clone!(content_state => move |evt:events::CustomChange| {
                    actions::rename_category(&content_state.cat, content_state.state.clone(), evt.value());
                }))
                .after_inserted(clone!(content_state => move |elem| {
                    *content_state.input_ref.borrow_mut() = Some(elem);
                }))
            })
        ];

        if content_state.parent.is_none() {
            children.push(html!("button-expand", {
                .property("slot", "content")
                .property("expanded", false)
                .event(clone!(content_state => move |evt:events::CustomToggle| {
                    actions::toggle_expand_all(&content_state.cat, evt.value());
                }))
            }));
        }

        children
        
    }
}

pub struct MenuDom {
}

impl MenuDom {
    pub fn render(content_state: Rc<ContentState>) -> Dom {
        html!("div", {
            .property("slot", "menu-content")
            .children(&mut [
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("add")
                    .event(clone!(content_state => move |evt:events::Click| {
                        actions::add_category_child(content_state.clone());
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("delete")
                    .event(clone!(content_state => move |evt:events::Click| {
                        actions::delete_category(content_state.clone());
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("move up")
                    .event(clone!(content_state => move |evt:events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Up);
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("move down")
                    .event(clone!(content_state => move |evt:events::Click| {
                        actions::move_category(content_state.clone(), actions::Direction::Down);
                    }))
                }),
                html!("button-rect", {
                    .property("kind", "text")
                    .property("color", "darkGray")
                    .property("hoverColor", "blue")
                    .text("rename")
                    .event(clone!(content_state => move |evt:events::Click| {
                        //These are only DOM changes
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
