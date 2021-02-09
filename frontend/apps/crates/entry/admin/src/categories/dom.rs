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

        html!("category-label", {
            .children(&mut [
                html!("button-expand", {
                    .property("slot", "expand")
                }),
                html!("category-button-add", {
                    .property("slot", "add")
                }),
                html!("div", {
                    .property("slot", "middle")
                    .children_signal_vec(state.categories.signal_vec_cloned().map(clone!(state => move |category| {
                        CategoryRoot::render(category, state.clone()) 
                    })))
                })

            ])
        })
    }
}

pub struct CategoryRoot {
}

impl CategoryRoot {
    pub fn render(cat: Rc<Category>, state: Rc<State>) -> Dom {
        html!("dropdown-tree", {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("open", cat.expanded.signal())
            .property_signal("mode", cat.editing.signal().map(|editing| {
                if editing { "textInput" } else { "textDisplay" }
            }))
            .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state => move |category| {
                CategoryLeaf::render(category, state.clone())
            })))
        })
    }
}

pub struct CategoryLeaf {
}

impl CategoryLeaf {
    pub fn render(cat: Rc<Category>, state: Rc<State>) -> Dom {
        html!("dropdown-tree-child", {
            .property_signal("label", cat.name.signal_cloned())
            .property_signal("open", cat.expanded.signal())
            .property_signal("mode", cat.editing.signal().map(|editing| {
                if editing { "textInput" } else { "textDisplay" }
            }))
            .children_signal_vec(cat.children.signal_vec_cloned().map(clone!(state => move |category| {
                CategoryLeaf::render(category, state.clone())
            })))
        })
    }
}
/*
 *    <category-label>
    
      <div slot="middle">
        ${mapToString(data, rootNode)}
        
      </div>
     
      <div slot="button">
      ${Rectangle({color:"red",size:"medium",contents:STR_PUBLISH,bold:false, italic:false})}
    </div>
    </category-label>
    */
