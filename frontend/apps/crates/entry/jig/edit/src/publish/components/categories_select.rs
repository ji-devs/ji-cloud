use std::rc::Rc;
use dominator::{Dom, clone, events, html};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use shared::domain::category::Category;
use utils::unwrap::UnwrapJiExt;

use crate::publish::state::State;

const STR_CATEGORIES_LABEL: &'static str = "Categories";
const STR_CATEGORIES_PLACEHOLDER: &'static str = "Select one or more";


pub fn render(state: Rc<State>) -> Dom {
    html!("div", {
        .property("slot", "catagories-select")
        .children(&mut [
            html!("p", {
                .text_signal(category_value_signal(state.clone()))
            }),
            html!("ul", {
                .property("slot", "catagories-select")
                .property("label", STR_CATEGORIES_LABEL)
                .property("placeholder", STR_CATEGORIES_PLACEHOLDER)
                .children_signal_vec(state.categories.signal_cloned().map(clone!(state => move |categories| {
                    match categories {
                        None => vec![],
                        Some(categories) => render_categories(state.clone(), &categories),
                    }
                })).to_signal_vec())
            }),
        ])
    })
    
}

fn render_categories(state: Rc<State>, categories: &Vec<Category>) -> Vec<Dom> {
    categories.iter().map(|category| {
        if category.children.len() == 0 {
            let category_id = category.id.clone();
            html!("li", {
                .style("color", "black")
                .text(&category.name)
                .text_signal(state.jig.categories.signal_cloned().map(clone!(category_id => move |selected_categories| {
                    match selected_categories.contains(&category_id) {
                        true => " ✅",
                        false => "",
                    }
                })))
                .event(clone!(state => move |_: events::Click| {
                    log::info!("open collections");
                    let mut categories = state.jig.categories.lock_mut();
                    if categories.contains(&category_id) {
                        categories.remove(&category_id);
                    } else {
                        categories.insert(category_id); 
                    }
                }))
            })
        } else {
            html!("li", {
                .text(&category.name)
                .style("color", "#a7a7a7")
                .child(html!("ul", {
                    .children(render_categories(state.clone(), &category.children))
                }))
            })
        }
    }).collect()
}

fn category_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_categories = state.jig.categories.signal_cloned(),
        let category_label_lookup = state.category_label_lookup.signal_cloned() => {
            let mut output = vec![];
            if let Some(category_label_lookup) = category_label_lookup {
                selected_categories.iter().for_each(|category_id| {
                    let category_name = category_label_lookup.get(category_id).unwrap_ji();
                    output.push(category_name.clone());
                })
            }
            output.join(", ")
        }
        
    }
}





































// const STR_CATEGORIES_LABEL: &'static str = "Categories";
// const STR_CATEGORIES_PLACEHOLDER: &'static str = "Select one or more";


// pub fn render(state: Rc<State>) -> Dom {
//     html!("dropdown-select", {
//         .property("slot", "catagories-select")
//         .property("label", STR_CATEGORIES_LABEL)
//         .property("placeholder", STR_CATEGORIES_PLACEHOLDER)
//         .children_signal_vec(state.categories.signal_cloned().map(clone!(state => move |categories| { // should be static?
//             match categories {
//                 None => vec![],
//                 Some(categories) => render_categories(state.clone(), categories),
//             }
//         })).to_signal_vec())
//     })
// }

// fn render_categories(state: Rc<State>, categories: Vec<Category>) -> Vec<Dom> {
//     categories.iter().map(|category| {
//         if category.children.len() == 0 {
//             let category_id = category.id.clone();
//             html!("li-check", {
//                 .text(&category.name)
//                 .event(clone!(state => move |_: events::Click| {
//                     log::info!("open collections");
//                     // state.jig.categories.lock_mut().push_cloned(category_id);
//                 }))
//             })
//         } else {
//             html!("li-check-collection" => HtmlElement, {
//                 .with_node!(_elem => {
//                     .text(&category.name)
//                     // .child(TooltipDom::render(TooltipData::Error(TooltipError {
//                     //     elem,
//                     //     placement: Placement::Bottom,
//                     //     slot: None,
//                     //     body: String::from("Please fill in the missing information."),
//                     //     max_width: None,
//                     //     on_close: None,
//                     //     move_strategy: MoveStrategy::Track,
//                     // })))
//                 })
//             })
//         }
//     }).collect()
// }
