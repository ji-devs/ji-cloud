use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use super::state::*;
use crate::tooltip::{
    callbacks::TooltipErrorCallbacks,
    state::{
        MoveStrategy, Anchor,ContentAnchor, State as TooltipState, TooltipData, TooltipError, TooltipTarget,
    },
};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};

pub fn render(state: Rc<State>) -> Dom {
    html!("sidebar-widget-dual-list", {
        .children(&mut [

            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .text(super::strings::STR_CLEAR)
                .event(clone!(state => move |_evt:events::Click| {
                    state.clear();
                }))
            }),
            html!("button-rect", {
                .property_signal("color", state.is_valid_signal().map(|ready| {
                    if ready.is_err() {
                        "lightGray"
                    } else {
                        "red"
                    }
                }))
                //.property_signal("disabled", state.is_valid_signal().map(|ready| ready.is_err()))
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "done-btn")
                .text(super::strings::STR_DONE)
                .event(clone!(state => move |_evt:events::Click| {
                    match state.derive_list() {
                        Ok(list) => {
                            (state.callbacks.replace_list) (list);
                        },
                        Err(_err) => {

                            (state.callbacks.set_tooltip_error) (Some(
                                    Rc::new(TooltipState::new(
                                        TooltipTarget::Element(
                                            state.error_element_ref.borrow().as_ref().unwrap_ji().clone(),
                                            MoveStrategy::None
                                        ),

                                        TooltipData::Error(Rc::new(TooltipError {
                                            max_width: Some(185.0),
                                            target_anchor: Anchor::MiddleRight,
                                            content_anchor: ContentAnchor::OppositeH,
                                            body: super::strings::error::STR_NUM_WORDS.to_string(),
                                            callbacks: TooltipErrorCallbacks::new(
                                                Some(clone!(state => move || {
                                                    (state.callbacks.set_tooltip_error) (None);
                                                }))
                                            )
                                        }))
                                    ))
                            ));
                        }
                    }
                }))
            }),
            render_column(state.clone(), ColumnSide::Left),
            render_column(state.clone(), ColumnSide::Right),
        ])
    })
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColumnSide {
    Left,
    Right,
}

impl ColumnSide {
    fn side_prop(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
    fn col_index(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }

    fn mutable(&self, state: &State) -> Rc<MutableVec<Mutable<String>>> {
        match self {
            Self::Left => state.left.clone(),
            Self::Right => state.right.clone(),
        }
    }
}

fn render_column(state: Rc<State>, side: ColumnSide) -> Dom {
    html!("sidebar-widget-dual-list-column", {
        .property("slot", side.side_prop())
        .property("side", side.side_prop())
        .property("header", &(state.callbacks.get_header) (side))
        .children_signal_vec(
            side.mutable(&state).signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, value)| {
                    let row = index.get().unwrap_or_default();
                    html!("sidebar-widget-dual-list-input", {
                        .property("rows", state.opts.cell_rows)
                        .property_signal("value", {
                            clone!(state => map_ref! {
                                let value = value.signal_cloned(),
                                let is_placeholder = state.is_placeholder.signal()
                                    => move {
                                        if *is_placeholder {
                                            (state.callbacks.get_placeholder) (row, side.col_index())
                                                .unwrap_or_else(|| "".to_string())
                                        } else {
                                            value.clone()
                                        }
                                    }
                            })
                        })
                        .property("constrain", state.callbacks.constrain.as_ref())
                        .property_signal("placeholder", state.is_placeholder.signal())
                        .event(clone!(state => move |_evt:events::Focus| {
                            //log::info!("got focus!");
                            state.is_placeholder.set_neq(false);
                        }))
                        .event(clone!(state => move |evt:events::CustomInput| {
                            value.set_neq(evt.value());
                        }))
                        .after_inserted(clone!(index, state, side => move |elem| {
                            if side == ColumnSide::Right && row == 2 {
                                *state.error_element_ref.borrow_mut() = Some(elem);
                            }

                        }))
                    })
                }))
        )
    })
}
