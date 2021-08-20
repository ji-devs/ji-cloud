use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;
use utils::{prelude::*, resize::ResizeInfo};

use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, SignalExt},
};

use super::{super::trace::state::*, menu::dom::render_menu};
use crate::traces::edit::state::*;

//see https://www.loom.com/share/c9ec53482ad94a97bff74d143a5a8cd2
pub fn render_select_box(
    state: Rc<TracesEdit>,
    index: ReadOnlyMutable<Option<usize>>,
    trace: &AllTrace,
    resize_info: &ResizeInfo,
) -> Dom {
    let select_box = trace.select_box.clone();

    let get_selected_signal = clone!(state, index => move || {
        map_ref! {
            let index = index.signal(),
            let selected = state.selected_index.signal()
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

    html!("empty-fragment", {

        .child_signal(
            select_box
                .menu_pos_signal(get_selected_signal())
                .map(clone!(state, select_box => move |pos| {
                    pos.map(|pos| {
                        html!("drag-container", {
                            .style("position", "fixed")
                            .style("top", "0")
                            .style("left", "0")
                            .property("x", pos.0 + 32.0)
                            .property("y", pos.1)
                            .child(html!("menu-container", {
                                .child(render_menu(state.clone(), index.clone()))
                            }))
                            .event(clone!(select_box => move |_evt:events::Close| {
                                log::info!("GOT CLOSE!");
                                select_box.menu_pos.set(None);
                            }))
                        })
                    })
                }))
        )
        .child_signal(trace.bounds.signal_cloned().map(clone!(resize_info, select_box => move |bounds| {
            bounds.map(|bounds| {
                //then draw our actual box
                let bounds = bounds.denormalize(&resize_info);
                html!("trace-edit-select-box", {
                    .property_signal("selected", get_selected_signal())
                    .style("left", &format!("{}px", bounds.x))
                    .style("top", &format!("{}px", bounds.y))
                    .style("width", &format!("{}px", bounds.width))
                    .style("height", &format!("{}px", bounds.height))

                    .event(clone!(state => move |evt:events::MouseDown| {
                        log::info!("mouse down...");
                        //item.start_drag(evt.x() as i32, evt.y() as i32);
                    }))
                    /*
                    .global_event_preventable(clone!(item => move |evt:events::MouseUp| {
                        //item.try_end_drag(evt.x() as i32, evt.y() as i32);

                    }))
                    .global_event_preventable(clone!(item => move |evt:events::MouseMove| {
                        //item.try_move_drag(evt.x() as i32, evt.y() as i32);
                    }))
                    */
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
