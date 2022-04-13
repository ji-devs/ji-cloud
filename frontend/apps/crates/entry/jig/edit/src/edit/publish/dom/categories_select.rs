use components::categories_input;
use dominator::{html, Dom};
use futures_signals::{map_ref, signal::Signal};
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

use super::super::state::Publish;

const STR_CATEGORIES_LABEL: &str = "Keywords";
const STR_CATEGORIES_PLACEHOLDER: &str = "Select one or more";

impl Publish {
    pub fn render_categories_select(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);

        html!("input-wrapper", {
            .property("slot", "catagories-select")
            .property("label", STR_CATEGORIES_LABEL)
            .child(categories_input::CategoriesInput::new(
                Box::pin(state.category_value_signal()),
                STR_CATEGORIES_PLACEHOLDER.to_string(),
                state.categories.get_cloned(),
                state.jig.categories.clone()
            ).render(None))
        })
    }

    fn category_value_signal(self: &Rc<Self>) -> impl Signal<Item = String> {
        let state = self;
        map_ref! {
            let selected_categories = state.jig.categories.signal_cloned(),
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
}
