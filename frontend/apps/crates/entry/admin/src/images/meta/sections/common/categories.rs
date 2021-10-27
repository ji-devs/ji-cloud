use dominator::{html, Dom};
use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::category::*;
use std::collections::HashSet;

pub struct MutableCategory {
    pub id: CategoryId,
    pub name: String,
    pub expanded: Mutable<bool>,
    pub children: Vec<Rc<MutableCategory>>,
}
impl From<Category> for MutableCategory {
    fn from(cat: Category) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            //only used in select view, but w/e
            expanded: Mutable::new(false),
            children: cat
                .children
                .into_iter()
                .map(|child| Rc::new(child.into()))
                .collect(),
        }
    }
}

pub fn category_selected(
    categories: Mutable<HashSet<CategoryId>>,
    cat: Rc<MutableCategory>,
) -> impl Signal<Item = bool> {
    categories.signal_ref(move |lookup| lookup.contains(&cat.id))
}

pub fn category_descendents_selected(
    categories: Mutable<HashSet<CategoryId>>,
    cat: Rc<MutableCategory>,
) -> impl Signal<Item = bool> {
    fn check(lookup: &HashSet<CategoryId>, cat: &Rc<MutableCategory>) -> bool {
        if lookup.contains(&cat.id) {
            true
        } else {
            cat.children.iter().any(|cat| check(lookup, cat))
        }
    }

    categories.signal_ref(move |lookup| check(lookup, &cat))
}

pub fn render_report(
    categories: Mutable<HashSet<CategoryId>>,
    parent: Option<Rc<MutableCategory>>,
    cat: Rc<MutableCategory>,
) -> Dom {
    html!("report-tree", {
        .style_signal("display", category_descendents_selected(categories.clone(), cat.clone()).map(|selected| {
            if selected { "block" } else { "none" }
        }))
        .children(&mut [
            html!("div", {
                .property("slot", "content")
                .text(&cat.name)
            })
        ])
        .property("hasChildren", !cat.children.is_empty())
        .property("isChild", parent.is_some())
        .child(html!("div", {
            .property("slot", "children")
            .children(cat.children.iter().map(|child| {
                render_report(categories.clone(), Some(cat.clone()), child.clone())
            }))
        }))
    })
}
