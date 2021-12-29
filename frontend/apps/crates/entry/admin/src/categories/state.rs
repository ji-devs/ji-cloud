use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use std::cell::RefCell;
use std::rc::Rc;

use super::debug;
use shared::domain::category::{Category as DbCategory, CategoryId};

use web_sys::HtmlElement;

pub struct State {
    pub categories: MutableVec<Rc<Category>>,
    pub deleting: Mutable<Option<Rc<ContentState>>>,
    pub loader: AsyncLoader,
}
impl State {
    pub fn new() -> Self {
        Self {
            categories: MutableVec::new(),
            deleting: Mutable::new(None),
            loader: AsyncLoader::new(),
        }
    }
}

pub struct Category {
    pub id: CategoryId,
    pub name: Mutable<String>,
    pub children: MutableVec<Rc<Category>>,
    pub expanded: Mutable<bool>,
    pub editing: Mutable<bool>,
}

impl Category {
    pub fn has_children_signal(&self) -> impl Signal<Item = bool> {
        self.children.signal_vec_cloned().len().map(|len| len > 0)
    }

    pub fn new(id: CategoryId, name: String, editing: bool) -> Self {
        Self::new_internal(id, name, None, editing)
    }
    pub fn new_with_children(id: CategoryId, name: String, children: Vec<Rc<Self>>) -> Self {
        Self::new_internal(id, name, Some(children), false)
    }

    fn new_internal(
        id: CategoryId,
        name: String,
        children: Option<Vec<Rc<Self>>>,
        editing: bool
    ) -> Self {
        Self {
            id,
            name: Mutable::new(name),
            children: match children {
                Some(children) => MutableVec::new_with_values(children),
                None => MutableVec::new(),
            },
            expanded: Mutable::new(debug::INIT_EXPANDED),
            editing: Mutable::new(editing),
        }
    }
}

impl From<DbCategory> for Category {
    fn from(cat: DbCategory) -> Self {
        let children: Vec<Rc<Self>> = cat
            .children
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
    //This is only needed for imperatively toggling via menu
    pub menu_ref: RefCell<Option<HtmlElement>>,
}
impl ContentState {
    pub fn new(parent: Option<Rc<Category>>, cat: Rc<Category>, state: Rc<State>) -> Self {
        Self {
            parent,
            cat,
            state,
            menu_ref: RefCell::new(None),
        }
    }
}
