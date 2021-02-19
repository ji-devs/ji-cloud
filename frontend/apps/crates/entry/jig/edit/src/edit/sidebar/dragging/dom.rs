use dominator::{html, Dom, clone};
use shared::domain::jig::Jig;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::edit::sidebar::state::State as SidebarState;
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use super::state::*;
use super::actions;

pub struct DraggingDom {
}

impl DraggingDom {
    pub fn render(sidebar: Rc<SidebarState>) -> Dom {
        html!("empty-fragment", {
            .child_signal(
                sidebar
                    .drag
                    .signal_cloned()
                    .map(|drag| {

                        //In order to avoid re-rendering the entire overlay on every movement
                        //We always return it when a drag is initialized
                        //Even if it hasn't passed the threshhold for actual dragging yet
                        //And tie the display style to the actual dragging state
                        drag.map(|state| {
                            let module = &state.module;

                            html!("jig-edit-sidebar-module", {
                                .future(state.inner.pos_signal().for_each(clone!(state => move |pos| {
                                    if let Some(pos) = pos {
                                        actions::update_index(state.clone(), pos.x, pos.y);
                                    }
                                    async {}
                                })))
                                .style_signal("display", state.inner.active_signal().map(|active| {
                                    if active { "block" } else { "none" }
                                }))
                                .style("position", "fixed")
                                .style("top", "0")
                                .style("left", "0")
                                .style("z-index", "9999")
                                .style_signal("transform", state.inner.transform_signal())
                                .property("selected", true)
                                .property("index", module.index as u32)
                                .property_signal("module", module.kind_str_signal())
                                .property("dragging", true)
                                .child(html!("jig-edit-sidebar-module-window", {
                                    .property("slot", "window")
                                    .property_signal("state", module.window_state_signal())
                                }))
                            })
                        })
                    })
            )
        })
    }
}
