use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};

use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, SignalExt},
};

use super::{super::trace::state::*, menu::dom::render_menu};
use crate::traces::edit::state::*;

//see https://www.loom.com/share/c9ec53482ad94a97bff74d143a5a8cd2

impl EditSelectTrace {
    pub fn render_select_box(
        parent: Rc<TracesEdit>,
        state: Rc<EditSelectTrace>,
        index: ReadOnlyMutable<Option<usize>>,
        resize_info: &ResizeInfo,
    ) -> Dom {
        let select_box = state.select_box.clone();

        let get_selected_signal = clone!(parent, state, index => move || {
            map_ref! {
                let index = index.signal(),
                let selected = parent.selected_index.signal()
                    => {
                        match (index, selected) {
                            (Some(index), Some(selected)) => {
                                *index == *selected
                            },
                            _ => false
                        }
                    }
            }
        });

        let bounds_index_signal = map_ref! {
            let bounds = select_box.bounds.signal_cloned(),
            let index = index.signal()
                => (bounds.clone(), *index)
        };

        html!("empty-fragment", {
            .future({

                //This should always be okay because the attribute_signal
                //to set the style on the shape itself
                //is bound to the same transform_override
                let sig = map_ref!{
                    let resize_info = resize_info_signal(),
                    let t = state.select_box.transform_override.signal_ref(|_| ())
                        => (resize_info.clone())
                };

                sig.for_each(clone!(state => move |resize_info| {
                    state.select_box.reset_bounds(&resize_info);
                    async move {}
                }))
            })
            .child_signal(
                select_box
                    .menu_pos_signal(get_selected_signal())
                    .map(clone!(parent, state, select_box => move |pos| {
                        pos.map(|pos| {
                            html!("drag-container", {
                                .style("position", "fixed")
                                .style("top", "0")
                                .style("left", "0")
                                .property("x", pos.0 + 32.0)
                                .property("y", pos.1)
                                .child(html!("menu-container", {
                                    .child(render_menu(parent.clone(), index.clone()))
                                }))
                                .event(clone!(select_box => move |_evt:events::Close| {
                                    log::info!("GOT CLOSE!");
                                    select_box.menu_pos.set(None);
                                }))
                            })
                        })
                    }))
            )
            .child_signal(bounds_index_signal.map(clone!(resize_info, select_box => move |(bounds, index)| {
                bounds.map(|bounds| {
                    //then draw our actual box
                    let bounds = bounds.denormalize(&resize_info);
                    html!("trace-edit-select-box", {
                        .property_signal("selected", get_selected_signal())
                        .style("left", &format!("{}px", bounds.x))
                        .style("top", &format!("{}px", bounds.y))
                        .style("width", &format!("{}px", bounds.width))
                        .style("height", &format!("{}px", bounds.height))

                        .child(
                            //Select area - to exclude the buttons
                            html!("div", {
                                .property("slot", "click-area")

                                .event(clone!(select_box => move |evt:events::MouseDown| {
                                    select_box.start_drag(evt.x() as i32, evt.y() as i32);
                                }))
                                .global_event_preventable(clone!(select_box => move |evt:events::MouseMove| {
                                    select_box.try_move_drag(evt.x() as i32, evt.y() as i32);
                                }))
                                .global_event_preventable(clone!(parent, select_box => move |evt:events::MouseUp| {
                                    if let Some(transform) = select_box.try_end_drag(evt.x() as i32, evt.y() as i32) {
                                        if let Some(index) = index {
                                            parent.change_transform(index, transform);
                                        }
                                    }
                                }))
                            })
                        )
                        .child(html!("button-icon" => web_sys::HtmlElement, {
                            .property("slot", "menu-btn")
                            .property("icon", "circle-kebab-grey")
                            .style("display", "block")
                            .with_node!(elem => {
                                .event(clone!(select_box => move |_evt:events::Click| {
                                    let dom_rect = elem.get_bounding_client_rect();
                                    let x = dom_rect.x();
                                    let y = dom_rect.y();
                                    select_box.menu_pos.set(Some((x, y)));
                                }))
                            })
                        }))
                    })
                })
            })))

        })
    }
}
