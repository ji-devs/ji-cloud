use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use futures_signals::signal::ReadOnlyMutable;

use crate::traces::edit::state::*;

pub fn render_menu(state: Rc<TracesEdit>, index: ReadOnlyMutable<Option<usize>>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "edit")
                .event(clone!(state, index => move |_evt:events::Click| {
                    if let Some(index) = index.get() {
                        TracesEdit::start_draw(state.clone(), Some(index), None);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(state, index => move |_evt:events::Click| {
                    if let Some(index) = index.get() {
                        state.duplicate(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(state, index => move |_evt:events::Click| {
                    if let Some(index) = index.get() {
                        state.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
