use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use super::state::*;

pub struct DualListDom {}
impl DualListDom {
    pub fn render(state: Rc<State>) -> Dom { 
        html!("sidebar-widget-dual-list", {
            .children(&mut [

                html!("button-text", {
                    .property("slot", "clear")
                    .text(crate::strings::STR_CLEAR)
                }),
                html!("button-sidebar", {
                    .property("slot", "input-buttons")
                    .property("mode", "keyboard")
                }),
                html!("button-sidebar", {
                    .property("slot", "input-buttons")
                    .property("mode", "dicta")
                }),
                html!("button-sidebar", {
                    .property("slot", "input-buttons")
                    .property("mode", "sefaria")
                }),
                html!("button-rect", {
                    .property("color", "grey")
                    .property("size", "small")
                    .property("iconAfter", "done")
                    .property("slot", "done-btn")
                    .text(crate::strings::STR_DONE)
                    .event(clone!(state => move |evt:events::Click| {
                        state.app.replace_dual_list(state.derive_list());
                    }))
                }),
                render_column(state.clone(), ColumnSide::Left),
                render_column(state.clone(), ColumnSide::Right),
            ])
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ColumnSide {
    Left,
    Right
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
    fn header(&self) -> &'static str {
        match self {
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
        }
    }
    fn mutable(&self, state:&State) -> Rc<MutableVec<Mutable<String>>> {
        match self {
            Self::Left => state.left.clone(),
            Self::Right => state.right.clone() 
        }
    }
}

fn render_column(state: Rc<State>, side: ColumnSide) -> Dom {
    html!("sidebar-widget-dual-list-column", {
        .property("side", side.side_prop())
        .property("header", side.header())
        .children_signal_vec(
            side.mutable(&state).signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, value)| {
                    let row = index.get().unwrap_or_default();
                    let mode = state.app.game_mode.get_cloned().unwrap_ji();

                    html!("sidebar-widget-dual-list-input", {
                        .property_signal("value", {
                            map_ref! {
                                let value = value.signal_cloned(),
                                let is_placeholder = state.is_placeholder.signal()
                                    => move {
                                        if *is_placeholder {
                                            match crate::config::get_dual_list_init_word(row, side.col_index()) {
                                                Some(s) => s.to_string(),
                                                None => "".to_string()
                                            }
                                        } else {
                                            value.clone()
                                        }
                                    }
                            }
                        })
                        .property_signal("placeholder", state.is_placeholder.signal())
                        .event(clone!(state => move |evt:events::Focus| {
                            //log::info!("got focus!");
                            state.is_placeholder.set_neq(false);
                        }))
                        .event(clone!(state => move |evt:events::CustomInput| {
                            value.set_neq(evt.value());
                        }))
                    })
                }))
        )
    })
}
/*
    <sidebar-widget-dual-list>
        <button-text slot="clear">${STR_CLEAR}</button-text>
        <button-sidebar slot="input-buttons" mode="keyboard"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="dicta"></button-sidebar>
        <button-sidebar slot="input-buttons" mode="sefaria"></button-sidebar>
        <button-rect color="grey" size="small" iconAfter="done" slot="done-btn">${STR_DONE}</button-rect>
        <sidebar-widget-dual-list-column side="left" header="${leftHeader}">
            ${mapToString(arrayCount(nRows), row => {

                const is_placeholder = row < placeholderCutoff;

                const value = is_placeholder ? "placeholder" : "value";
                const placeholder = is_placeholder ? "placeholder" : "";

                return`<sidebar-widget-dual-list-input value="${value}" nLines="${nLines}" ${placeholder}></sidebar-widget-dual-list-input>`
            })}
        </sidebar-widget-dual-list-column>
        <sidebar-widget-dual-list-column side="right" header="${rightHeader}">
            ${mapToString(arrayCount(nRows), row => {

                const is_placeholder = row < placeholderCutoff;

                const value = is_placeholder ? "placeholder" : "value";
                const placeholder = is_placeholder ? "placeholder" : "";

                return`<sidebar-widget-dual-list-input value="${value}" nLines="${nLines}" ${placeholder}></sidebar-widget-dual-list-input>`
            })}
        </sidebar-widget-dual-list-column>
    </sidebar-widget-dual-list>`
    */
