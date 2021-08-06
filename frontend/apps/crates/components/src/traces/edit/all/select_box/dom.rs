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
        /*
         * Now using the real SVG path
         * though it doesn't seem to make much of a difference
        .child({
            //First throw a dummy div over where the transform box would be
            //it's totally transparent, just to enable getting a DomRect easily
            let bounds = trace.calc_bounds(false).unwrap_ji();
            let bounds = bounds.denormalize(resize_info);
            html!("div" => web_sys::HtmlElement, {
                .style("pointer-events", "none")
                .style("position", "absolute")
                .style("opacity", "0")
                .style("transform", trace.transform.denormalize_matrix_string(resize_info))
                .style("left", &format!("{}px", bounds.x))
                .style("top", &format!("{}px", bounds.y))
                .style("width", &format!("{}px", bounds.width))
                .style("height", &format!("{}px", bounds.height))
                .after_inserted(clone!(select_box, resize_info => move |elem| {
                    //stash the domrect as a BoundsF64 of normalized coordinates
                    //technically we re-render this whenever resize_info changes
                    //but it's still a bit cleaner to stash this way
                    let rect = elem.get_bounding_client_rect();
                    select_box.bounds.set(Some(BoundsF64::new_from_dom_normalized(&rect, &resize_info)));
                }))
                .after_removed(clone!(select_box => move |elem| {
                    select_box.bounds.set(None);
                }))
            })
        })
        */
        //can get rid of this nesting with dominator update
        .child(html!("empty-fragment", {

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
        }))
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
