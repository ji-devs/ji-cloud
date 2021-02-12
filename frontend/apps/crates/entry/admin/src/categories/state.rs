use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use shared::domain::category::{CategoryId, Category as DbCategory};
use super::debug;
use std::collections::HashSet;
use web_sys::HtmlElement;



pub struct State {
    pub categories: MutableVec<Rc<Category>>,
    pub loader: AsyncLoader
}
impl State {
    pub fn new() -> Self {
        Self {
            categories: MutableVec::new(),
            loader: AsyncLoader::new(),
        }
    }
}

pub struct Category {
    pub id: CategoryId,
    pub name: Mutable<String>,
    pub children: MutableVec<Rc<Category>>,
    pub expanded: Mutable<bool>,
}

impl Category {
    pub fn has_children_signal(&self) -> impl Signal<Item = bool> {
        self
            .children
            .signal_vec_cloned()
            .len()
            .map(|len| if len > 0 { true } else { false })
    }

    pub fn new(id: CategoryId, name: String) -> Self {
        Self::_new(id, name, None)
    }
    pub fn new_with_children(id: CategoryId, name: String, children: Vec<Rc<Self>>) -> Self {
        Self::_new(id, name, Some(children))
    }

    fn _new(id: CategoryId, name: String, children: Option<Vec<Rc<Self>>>) -> Self {
        Self {
            id,
            name: Mutable::new(name),
            children: match children {
                Some(children) => MutableVec::new_with_values(children),
                None => MutableVec::new()
            },
            expanded: Mutable::new(debug::INIT_EXPANDED),
        }
    }
}

impl From<DbCategory> for Category {
    fn from(cat:DbCategory) -> Self {

        let children:Vec<Rc<Self>> = cat.children
            .into_iter()
            .map(Category::from)
            .map(Rc::new)
            .collect();

        
        Self::new_with_children(cat.id, cat.name, children)
    }
}


pub struct ContentState {
    pub parent: Option<Rc<Category>>, 
    pub cat: Rc<Category>, 
    pub state: Rc<State>, 
    //These are only needed for imperatively toggling via menu
    pub input_ref: RefCell<Option<HtmlElement>>,
    pub menu_ref: RefCell<Option<HtmlElement>>
}
impl ContentState {
    pub fn new(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Self {
        Self {
            parent,
            cat,
            state,
            input_ref: RefCell::new(None),
            menu_ref: RefCell::new(None),
        }
    }
}

