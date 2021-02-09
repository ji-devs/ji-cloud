use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};

pub struct CategoriesPage {
}

impl CategoriesPage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_categories(state.clone());
        
        html!("empty-fragment", {
            .child(
                html!("category-label", {
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
                                CategoryDom::render(category, state.clone(), true) 
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
    pub fn render(cat: Rc<Category>, state: Rc<State>, is_root: bool) -> Dom {
        let element_name = if is_root { "dropdown-tree" } else { "dropdown-tree-child" };

        html!(element_name, {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("expanded", cat.expanded.signal())
            .property_signal("hasChildren", cat.has_children_signal())
            .property_signal("mode", cat.editing.signal().map(|editing| {
                if editing { "textInput" } else { "textDisplay" }
            }))
            .event(clone!(cat => move |evt:events::ExpandAll| {
                actions::toggle_expand_all(&cat, true)
            }))
            .event(clone!(cat => move |evt:events::CollapseAll| {
                actions::toggle_expand_all(&cat, false)
            }))
            .event(clone!(cat => move |evt:events::DoubleClick| {
                actions::handle_double_click(&cat);
            }))
            .event(clone!(cat => move |evt:events::Change| {
                log::info!("CHANGE EVENT");
            }))
            .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state => move |category| {
                CategoryDom::render(category, state.clone(), false)
            })))
        })
    }
}
