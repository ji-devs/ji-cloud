use dominator::{html, clone, Dom};
use std::rc::Rc;
use crate::images::meta::state::{State as MetaState, MutableImage};
use super::{state::*, actions};
use utils::{events, routes::*, api_helpers::meta::MetaOptions};
use wasm_bindgen::prelude::*;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use shared::domain::category::Category;
use crate::images::meta::sections::common::categories::*;

pub struct CategoriesDom {
}

impl CategoriesDom {
    pub fn render(meta_state: Rc<MetaState>, image: Rc<MutableImage>, categories: Rc<Vec<Rc<MutableCategory>>>) -> Dom {
        let state = Rc::new(State::new(meta_state, image, categories));


        html!("image-meta-section-categories", {
            .children(&mut [
                html!("div", {
                    .property("slot", "category-select")
                    .children(state.categories.iter().map(|cat| {
                        render_select(None, cat.clone(), state.clone())
                    }))
                }),
                html!("div", {
                    .property("slot", "category-report")
                    .children(state.categories.iter().map(|cat| {
                        render_report(state.image.categories.clone(), None, cat.clone())
                    }))
                }),
                html!("button-expand", {
                    .property("slot", "expand")
                    .property("expanded", false)
                    .event(clone!(state => move |evt:events::CustomToggle| {
                        let flag = evt.value();
                        for cat in state.categories.iter() {
                            actions::toggle_expand_all(&cat, flag);
                        }
                    }))
                })
            ])
        })
    }
}


pub fn render_select(parent: Option<Rc<MutableCategory>>, cat: Rc<MutableCategory>, state: Rc<State>) -> Dom {


    let has_children = if cat.children.len() > 0 { true } else { false };

    let mut content:Vec<Dom> = vec![];

    if has_children {
        content.push(
            html!("div", {
                .property("slot", "content")
                .text(&cat.name)
            })
        );
    } else {
        content.push(
            html!("input-checkbox", {
                .property("slot", "content")
                .property("label", &cat.name)
                .property_signal("checked", category_selected(state.image.categories.clone(), cat.clone()))
                .event(clone!(cat, state => move |evt:events::CustomToggle| {
                    actions::on_toggle(cat.id.clone(), state.clone(), evt.value());
                }))
            })
        );
    }

    if parent.is_none() && has_children {
        content.push(html!("button-expand", {
            .property("slot", "content")
            .property("expanded", false)
            .event(clone!(cat => move |evt:events::CustomToggle| {
                actions::toggle_expand_all(&cat, evt.value());
            }))
        }));
    }

    html!("dropdown-tree", {
        .property_signal("expanded", cat.expanded.signal())
        .property("hasChildren", has_children)
        .property("isChild", parent.is_some())
        .event(clone!(cat => move |evt:events::ExpandAll| {
            actions::toggle_expand_all(&cat, true);
        }))
        .event(clone!(cat => move |evt:events::CollapseAll| {
            actions::toggle_expand_all(&cat, false);
        }))
        .children(content)
        .child(html!("div", {
            .property("slot", "children")
            .children(cat.children.iter().map(|child| {
                render_select(Some(cat.clone()), child.clone(), state.clone())
            }))
        }))
    })
}

