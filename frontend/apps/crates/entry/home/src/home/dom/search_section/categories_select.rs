use components::categories_input;
use dominator::{html, Dom, clone};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

use super::super::super::state::State;

const STR_CATEGORIES_LABEL: &str = "Categories";
const STR_CATEGORIES_PLACEHOLDER: &str = "Select one or more from the list";

pub fn render(state: Rc<State>) -> Dom {
    html!("input-wrapper", {
        .property("slot", "categories")
        .property("label", STR_CATEGORIES_LABEL)
        .child_signal(state.search_options.categories.signal_cloned().map(clone!(state => move |category_options| {
            Some(categories_input::CategoriesInput::new(
                Box::pin(category_value_signal(Rc::clone(&state))),
                STR_CATEGORIES_PLACEHOLDER.to_string(),
                category_options,
                state.search_selected.categories.clone()
            ).render(None))
        })))
    })
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
