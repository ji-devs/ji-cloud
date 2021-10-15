use shared::domain::{category::*, image::*};

use super::state::*;
use std::rc::Rc;

use crate::images::meta::sections::common::categories::*;

pub fn toggle_expand_all(cat: &Rc<MutableCategory>, flag: bool) {
    cat.expanded.set(flag);

    for child in cat.children.iter() {
        toggle_expand_all(child, flag);
    }
}

pub fn on_toggle(id: CategoryId, state: Rc<State>, flag: bool) {
    {
        let mut categories = state.image.categories.lock_mut();
        if flag {
            categories.insert(id);
        } else {
            categories.remove(&id);
        }
    }

    crate::images::meta::actions::save(
        state.meta.clone(),
        ImageUpdateRequest {
            categories: Some(
                state
                    .image
                    .categories
                    .lock_ref()
                    .iter()
                    .map(|x| x.clone())
                    .collect(),
            ),
            ..ImageUpdateRequest::default()
        },
    );
}
