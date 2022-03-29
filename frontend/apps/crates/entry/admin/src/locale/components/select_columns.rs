use super::super::state::{Column, State};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableSignalVec, MutableVec},
};
use std::rc::Rc;
use utils::events;
use utils::unwrap::UnwrapJiExt;

fn render_list(
    slot: &str,
    list: MutableSignalVec<Column>,
    selected: Mutable<Option<Column>>,
) -> Dom {
    html!("div", {
        .style("display", "contents")
        .property("slot", slot)
        .children_signal_vec(list
            .map(clone!(selected => move |column: Column| html!("locale-select-columns-item", {
                .text(&column.to_string())
                .property_signal("active", selected.signal_cloned().map(clone!(column => move |e| {
                    e.is_some() && e.unwrap_ji() == column
                })))
                .event(clone!(selected, column => move |_: events::Click| {
                    if selected.lock_ref().is_none() || &selected.lock_ref().clone().unwrap_ji() != &column {
                        selected.set(Some(column.clone()));
                    } else {
                        selected.set(None);
                    }
                }))
            })))
        )
    })
}

fn render_move_button(
    button_content: &str,
    list: Rc<MutableVec<Column>>,
    selected: Mutable<Option<Column>>,
    other_list: Rc<MutableVec<Column>>,
) -> Dom {
    html!("button-rect", {
        .text(button_content)
        .property("slot", "move-actions")
        .property("kind", "text")
        .property("weight", "bold")
        .event(clone!(selected, list, other_list => move |_: events::Click| {
            let selected_ref = selected.lock_ref().clone();
            if selected_ref.is_some() {
                let selected_ref = selected_ref.unwrap_ji();
                let mut list = list.lock_mut();
                let pos = list.iter().position(|i| i == &selected_ref).unwrap_ji();
                list.remove(pos);
                other_list.lock_mut().push_cloned(selected_ref);
                selected.set(None);
            }
        }))
    })
}

fn render_sort_button(
    button_content: &str,
    list: Rc<MutableVec<Column>>,
    selected: Mutable<Option<Column>>,
    on_click: fn(usize, usize) -> Option<usize>,
) -> Dom {
    html!("button-rect", {
        .text(button_content)
        .property("slot", "sort-actions")
        .property("kind", "text")
        .property("weight", "bold")
        .event(clone!(selected, list => move |_: events::Click| {
            let selected_ref = selected.lock_ref().clone();
            if selected_ref.is_some() {
                let selected_ref = selected_ref.unwrap_ji();
                let mut list = list.lock_mut();
                let pos = list.iter().position(|i| i == &selected_ref).unwrap_ji();

                let new_pos = on_click(pos, list.len());
                if new_pos.is_some() {
                    let v = list.remove(pos);
                    list.insert_cloned(new_pos.unwrap_ji(), v);
                }
            }
        }))
    })
}

pub fn render(state: Rc<State>) -> Dom {
    let visible_selected: Mutable<Option<Column>> = Mutable::new(None);
    let hidden_selected: Mutable<Option<Column>> = Mutable::new(None);

    html!("locale-select-columns", {
        .property("slot", "dialog-content")
        .children(&mut [
            render_list("hidden-columns", state.hidden_columns.clone().signal_vec_cloned(), hidden_selected.clone()),
            render_move_button(
                "⇨",
                state.hidden_columns.clone(),
                hidden_selected,
                state.visible_columns.clone()
            ),
            render_move_button(
                "⇦",
                state.visible_columns.clone(),
                visible_selected.clone(),
                state.hidden_columns.clone()
            ),
            render_list("visible-columns", state.visible_columns.clone().signal_vec_cloned(), visible_selected.clone()),
            render_sort_button(
                "⇧",
                state.visible_columns.clone(),
                visible_selected.clone(),
                |pos: usize, _| match pos {
                    0 => None,
                    _ => Some(pos - 1)
                }
            ),
            render_sort_button(
                "⇩",
                state.visible_columns.clone(),
                visible_selected,
                |pos: usize, list_len| {
                    if pos >= list_len - 1 {
                        None
                    } else {
                        Some(pos + 1)
                    }
                }
            )
        ])
    })
}
