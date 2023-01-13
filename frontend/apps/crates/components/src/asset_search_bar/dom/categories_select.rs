use crate::categories_input;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{from_future, Signal, SignalExt},
};
use std::rc::Rc;
use utils::{
    metadata::{get_categories, get_category_label_lookup},
    unwrap::UnwrapJiExt,
};

use super::AssetSearchBar;

const STR_CATEGORIES_LABEL: &str = "Categories";
const STR_CATEGORIES_PLACEHOLDER: &str = "Select one or more from the list";

pub fn render(state: Rc<AssetSearchBar>) -> Dom {
    html!("input-wrapper", {
        .prop("slot", "categories")
        .prop("label", STR_CATEGORIES_LABEL)
        .child_signal(from_future(get_categories()).map(clone!(state => move |category_options| {
            category_options.map(|category_options| {
                categories_input::CategoriesInput::new(
                    Box::pin(category_value_signal(Rc::clone(&state))),
                    STR_CATEGORIES_PLACEHOLDER.to_string(),
                    (*category_options).clone(),
                    state.search_selected.categories.clone()
                ).render(None)
            })
        })))
    })
}

fn category_value_signal(state: Rc<AssetSearchBar>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_categories = state.search_selected.categories.signal_cloned(),
        let category_label_lookup = from_future(get_category_label_lookup()).map(|x| x.unwrap_or_default()) => {
            let mut output = vec![];
            selected_categories.iter().for_each(|category_id| {
                let category_name = category_label_lookup.get(category_id).unwrap_ji();
                output.push(category_name.clone());
            });
            output.join(", ")
        }
    }
}
