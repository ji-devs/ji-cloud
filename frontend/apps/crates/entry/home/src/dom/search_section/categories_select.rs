use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::category::Category;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

use crate::state::State;

const STR_CATEGORIES_LABEL: &'static str = "Categories";
const STR_CATEGORIES_PLACEHOLDER: &'static str = "Select one or more from the list";

pub fn render(state: Rc<State>) -> Dom {
    html!("input-select", {
        .property("slot", "categories")
        .property("label", STR_CATEGORIES_LABEL)
        .property("placeholder", STR_CATEGORIES_PLACEHOLDER)
        .property("multiple", true)
        .property_signal("value", category_value_signal(state.clone()))
        .children_signal_vec(state.search_options.categories.signal_cloned().map(clone!(state => move |categories| {
            render_categories(state.clone(), &categories)
        })).to_signal_vec())
    })
}

fn render_categories(state: Rc<State>, categories: &Vec<Category>) -> Vec<Dom> {
    categories.iter().map(|category| {
        if category.children.len() == 0 {
            let category_id = category.id.clone();
            html!("input-select-option", {
                .text(&category.name)
                .property_signal("selected", state.search_selected.categories.signal_cloned().map(clone!(category_id => move |selected_categories| {
                    selected_categories.contains(&category_id)
                })))
                .event(clone!(state => move |_: events::CustomSelectedChange| {
                    let mut categories = state.search_selected.categories.lock_mut();
                    if categories.contains(&category_id) {
                        categories.remove(&category_id);
                    } else {
                        categories.insert(category_id);
                    };
                }))
            })
        } else {
            html!("input-select-option-group", {
                .child(html!("span", {
                    .property("slot", "label")
                    .text(&category.name)
                }))
                .children(render_categories(state.clone(), &category.children))
            })
        }
    }).collect()
}

fn category_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_categories = state.search_selected.categories.signal_cloned(),
        let category_label_lookup = state.search_options.category_label_lookup.signal_cloned() => {
            let mut output = vec![];
            selected_categories.iter().for_each(|category_id| {
                let category_name = category_label_lookup.get(category_id).unwrap_ji();
                output.push(category_name.clone());
            });
            output.join(", ")
        }
    }
}
