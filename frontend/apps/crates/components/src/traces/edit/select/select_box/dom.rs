use dominator::{clone, html, with_node, Dom, DomBuilder};
use std::rc::Rc;
use web_sys::HtmlElement;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}};

use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, SignalExt},
};

use super::{super::trace::state::*, menu::dom::render_menu};
use crate::{
    traces::edit::state::*,
    box_outline::{BoxOutline,BoxOutlineMixins},
    buttons::{Button, ButtonStyle, ButtonStyleIcon}
};
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

        let selected_has_bounds_index_signal = map_ref! {
            let selected = get_selected_signal(), 
            let has_bounds = select_box.bounds.signal_cloned().map(|b| b.is_some()).dedupe(),
            let index = index.signal()
                => (*selected, *has_bounds, *index)
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
                                    select_box.menu_pos.set(None);
                                }))
                            })
                        })
                    }))
            )
            .child_signal(selected_has_bounds_index_signal.map(clone!(parent, resize_info, select_box => move |(is_selected, has_bounds, index)| {
                if !is_selected || !has_bounds {
                    None
                } else {
                    Some(
                        BoxOutline::render_mixins(
                                BoxOutline::new(
                                    clone!(resize_info, select_box => move || select_box.bounds.signal_cloned().map(clone!(resize_info => move |bounds| {
                                        bounds.unwrap_ji().denormalize(&resize_info)
                                    })))
                                ),
                                None,
                                BoxOutlineMixins {
                                    //handle movement
                                    click_area: Some(clone!(parent, select_box => move |dom:DomBuilder<HtmlElement>| {
                                        dom
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
                                    })),

                                    //adds a menu button to the top-right corner 
                                    top_right: Some(clone!(resize_info, select_box => move |dom:DomBuilder<HtmlElement>| {
                                        dom
                                            .child(
                                                Button::render(
                                                    Button::new(
                                                        ButtonStyle::Icon(ButtonStyleIcon::GreyKebab),
                                                        clone!(resize_info, select_box => move || { 
                                                            let bounds = select_box.bounds.get_cloned().unwrap_ji().denormalize(&resize_info);

                                                            let (x, y) = resize_info.get_fixed_pos_px(bounds.x + bounds.width, bounds.y);

                                                            select_box.menu_pos.set(Some((x, y)));
                                                        })
                                                    ),
                                                    None
                                                )
                                            )
                                    }))
                                }
                        ))
                }
            })))
        })
    }
}
