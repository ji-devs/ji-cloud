use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::category::CategoryId;
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    html!("div", {
        .property("slot", "category-labels")
        .children_signal_vec(state.jig.categories.signal_cloned().map(clone!(state => move|categories| {
            categories.iter()
                .map(|category_id| render_pill(state.clone(), category_id.clone()))
                .collect()
        })).to_signal_vec())
    })
}

fn render_pill(state: Rc<State>, category_id: CategoryId) -> Dom {
    html!("pill-close", {
        .property_signal("label", state.category_label_lookup.signal_cloned().map(move |category_label_lookup| {
            match category_label_lookup {
                None => String::new(),
                Some(category_label_lookup) => {
                    log::info!("{:?}", category_label_lookup.get(&category_id));
                    category_label_lookup.get(&category_id).unwrap_ji().clone()
                }
            }
        }))
    })
}
