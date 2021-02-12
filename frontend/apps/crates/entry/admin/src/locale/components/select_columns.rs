use web_sys::HtmlDialogElement;
use crate::locale::state::State;
use std::rc::Rc;
use dominator::{Dom, html, clone, with_node, events};
use futures_signals::signal_vec::SignalVecExt;


pub struct SelectColumns {

}

impl SelectColumns {
    pub fn render(state: Rc<State>) -> Dom {
        html!("dialog" => HtmlDialogElement, {
            .with_node!(element => {
                .after_inserted(clone!(state => move |element| {
                    *state.dialog_ref.lock_mut() = Some(element);
                }))
                .after_removed(clone!(state => move |_element| {
                    *state.dialog_ref.lock_mut() = None;
                }))
            })
            .child(html!("div", {
                .class("column-selection-contents")
                .children(&mut [
                    html!("header", {
                        .text("Select Fields to Display")
                    }),
                    html!("hr"),
                    html!("ul", {
                        .class("columns-hidden")
                        .children_signal_vec(
                            state.hidden_columns.signal_vec_cloned()
                                .map(|column| {
                                    html!("li", {
                                        .text(&column)
                                    })
                                })
                        )
                    }),
                    html!("ul", {
                        .class("columns-visible")
                        .children_signal_vec(
                            state.visible_columns.signal_vec_cloned()
                                .map(|column| {
                                    html!("li", {
                                        .text(&column)
                                    })
                                })
                        )
                    }),
                    html!("hr"),
                    html!("div", {
                        .class("actions")
                        .children(&mut [
                            html!("button", {
                                .text("Cancel")
                                .event(clone!(state => move |_event: events::Click| {
                                    state.dialog_ref
                                        .lock_ref().clone().unwrap()
                                        .close();
                                }))
                            }),
                            html!("button", {
                                .text("Save")
                                .event(clone!(state => move |_event: events::Click| {
                                    state.dialog_ref
                                        .lock_ref().clone().unwrap()
                                        .close();
                                }))
                            }),
                        ])
                    }),
                ])
            }))
        })
    }
}
