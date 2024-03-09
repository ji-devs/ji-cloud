use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::category::CategoryId;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

use super::super::state::Publish;

impl Publish {
    pub fn render_category_pills(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("div", {
            .prop("slot", "category-labels")
            .children_signal_vec(state.playlist.categories.signal_cloned().map(clone!(state => move|categories| {
                categories.iter()
                    .map(|category_id| render_pill(state.clone(), category_id.clone()))
                    .collect()
            })).to_signal_vec())
        })
    }
}

fn render_pill(state: Rc<Publish>, category_id: CategoryId) -> Dom {
    html!("pill-close", {
        .prop_signal("label", state.category_label_lookup.signal_cloned().map(move |category_label_lookup| {
            category_label_lookup.get(&category_id).unwrap_ji().clone()
        }))
        .child(html!("pill-close-delete", {
            .prop("slot", "delete")
            .event(clone!(state => move |_: events::Click| {
                state.playlist.categories.lock_mut().remove(&category_id);
            }))
        }))
    })
}
