use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::category::Category;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

use super::super::state::Publish;

const STR_CATEGORIES_LABEL: &'static str = "Categories";
const STR_CATEGORIES_PLACEHOLDER: &'static str = "Select one or more";

impl Publish {
    pub fn render_categories_select(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("input-select", {
            .prop("slot", "catagories-select")
            .prop("label", STR_CATEGORIES_LABEL)
            .prop("placeholder", STR_CATEGORIES_PLACEHOLDER)
            .prop("multiple", true)
            .prop_signal("value", category_value_signal(state.clone()))
            .prop_signal("error", {
                (map_ref! {
                    let submission_tried = state.submission_tried.signal(),
                    let value = state.playlist.categories.signal_cloned()
                        => (*submission_tried, value.clone())
                })
                    .map(|(submission_tried, value)| {
                        submission_tried && value.is_empty()
                    })
            })
            .children_signal_vec(state.categories.signal_cloned().map(clone!(state => move |categories| {
                render_categories(state.clone(), &categories)
            })).to_signal_vec())
        })
    }
}

fn render_categories(state: Rc<Publish>, categories: &Vec<Category>) -> Vec<Dom> {
    categories.iter().map(|category| {
        if category.children.len() == 0 {
            let category_id = category.id.clone();
            html!("input-select-option", {
                .text(&category.name)
                .prop_signal("selected", state.playlist.categories.signal_cloned().map(clone!(category_id => move |selected_categories| {
                    selected_categories.contains(&category_id)
                })))
                .event(clone!(state => move |_: events::CustomSelectedChange| {
                    let mut categories = state.playlist.categories.lock_mut();
                    if categories.contains(&category_id) {
                        categories.remove(&category_id);
                    } else {
                        categories.insert(category_id);
                    }
                }))
            })
        } else {
            html!("input-select-option-group", {
                .child(html!("span", {
                    .prop("slot", "label")
                    .text(&category.name)
                }))
                .children(render_categories(state.clone(), &category.children))
            })
        }
    }).collect()
}

fn category_value_signal(state: Rc<Publish>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_categories = state.playlist.categories.signal_cloned(),
        let category_label_lookup = state.category_label_lookup.signal_cloned() => {

            let len = selected_categories.len();
            if len == 0 {
                String::new()
            } else {
                let category = selected_categories.iter().next().unwrap_ji();
                let first_category_label = category_label_lookup.get(category).unwrap_ji().clone();
                if len == 1 {
                    first_category_label
                } else {
                    format!("{} +{}", first_category_label, len-1)
                }
            }

        }
    }
}
