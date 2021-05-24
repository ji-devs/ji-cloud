use std::rc::Rc;
use dominator::{Dom, clone, events, html};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use shared::domain::category::Category;

use crate::state::State;


const STR_CATEGORIES_LABEL: &'static str = "Categories";


pub fn render(state: Rc<State>) -> Dom {
    html!("dropdown-select", {
        .property("slot", "catagories-select")
        .property("label", STR_CATEGORIES_LABEL)
        .property("nested", true)
        // .property_signal("value", category_value_signal(state.clone()))
        .children_signal_vec(state.search_options.categories.signal_cloned().map(clone!(state => move |categories| {
            render_categories(state.clone(), &categories)
        })).to_signal_vec())
    })
    
}

fn render_categories(state: Rc<State>, categories: &Vec<Category>) -> Vec<Dom> {
    categories.iter().map(|category| {
        if category.children.len() == 0 {
            let category_id = category.id.clone();
            html!("li-check", {
                .text(&category.name)
                .property_signal("selected", state.search_selected.categories.signal_cloned().map(clone!(category_id => move |selected_categories| {
                    selected_categories.contains(&category_id)
                })))
                .event(clone!(state => move |_: events::Click| {
                    let mut categories = state.search_selected.categories.lock_mut();
                    if categories.contains(&category_id) {
                        categories.remove(&category_id);
                    } else {
                        categories.insert(category_id); 
                    };
                }))
            })
        } else {
            html!("li-check-collection", {
                .child(html!("span", {
                    .property("slot", "label")
                    .text(&category.name)
                }))
                .children(render_categories(state.clone(), &category.children))
            })
        }
    }).collect()
}

// fn category_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
//     map_ref! {
//         let selected_categories = state.search_options.categories.signal_cloned(),
//         let category_label_lookup = state.category_label_lookup.signal_cloned() => {
//             let mut output = vec![];
//             if let Some(category_label_lookup) = category_label_lookup {
//                 selected_categories.iter().for_each(|category_id| {
//                     let category_name = category_label_lookup.get(category_id).unwrap_ji();
//                     output.push(category_name.clone());
//                 })
//             }
//             output.join(", ")
//         }
//     }
// }
