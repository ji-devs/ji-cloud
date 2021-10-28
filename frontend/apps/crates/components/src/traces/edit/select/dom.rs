use dominator::{clone, html, Dom, DomBuilder};
use std::rc::Rc;
use utils::resize::{resize_info_signal, ResizeInfo};

use super::trace::state::*;
use crate::traces::{
    edit::state::*,
    svg::{self, ShapeStyle, ShapeStyleKind, ShapeStyleMode, ShapeStyleEditMode, ShapeStyleVar, SvgCallbacks, TransformSize},
};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};

use shared::domain::jig::module::body::_groups::design::TraceShape;
use web_sys::SvgElement;

impl TracesEdit {
    pub fn render_selectable(state: Rc<Self>) -> Dom {
        let mask_children =
            resize_info_signal().switch_signal_vec(clone!(state => move |resize_info| {
                state.list
                    .signal_vec_cloned()
                    .map(clone!(resize_info => move |trace| {
                        let shape_style = ShapeStyleVar::new_static(ShapeStyle::new_mask());
                        let callbacks = SvgCallbacks::new(
                            None::<fn()>,
                            None::<fn(web_sys::SvgElement)>,
                            None::<fn(web_sys::SvgElement)>,
                            None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
                        );
                        render_trace(shape_style, &resize_info, &trace, callbacks)
                    }))
            }));

        let draw_children =
            resize_info_signal().switch_signal_vec(clone!(state => move |resize_info| {
                state.list
                    .signal_vec_cloned()
                    .enumerate()
                    .map(clone!(resize_info, state => move |(index, trace)| {
                        let style_kind:ShapeStyleKind = trace.kind.into();

                        let shape_style_signal = map_ref!{
                            let selected_index = state.selected_index.signal_cloned(),
                            let index = index.signal_cloned()
                                => {
                                    (*selected_index, *index)
                                }
                        }.map(move |(selected_index, index)| {
                            ShapeStyle {
                                interactive: true,
                                mode: ShapeStyleMode::Edit(
                                    if index == selected_index {
                                        ShapeStyleEditMode::Selected
                                    } else {
                                        ShapeStyleEditMode::Deselected
                                    }
                                ),
                                kind: style_kind,
                            }
                        });

                        let shape_style = ShapeStyleVar::Dynamic(shape_style_signal);

                        let callbacks = SvgCallbacks::new(
                            //I don't think this is actually being called anymore
                            //since select_box is _always_ visible now
                            Some(clone!(state, index => move || {
                                if let Some(index) = index.get_cloned() {
                                    state.select_index(index);
                                }
                            })),
                            Some(clone!(trace => move |elem:SvgElement| {
                                *trace.select_box.elem.borrow_mut() = Some(elem);
                            })),
                            Some(clone!(trace => move |_elem| {
                                *trace.select_box.elem.borrow_mut() = None;
                                trace.select_box.bounds.set(None);
                            })),
                            None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
                        );
                        render_trace(shape_style, &resize_info, &trace, callbacks)
                    }))
            }));

        let menu_children =
            resize_info_signal().switch_signal_vec(clone!(state => move |resize_info| {
                state.list
                    .signal_vec_cloned()
                    .enumerate()
                    .map(clone!(state, resize_info => move |(index, trace)| {
                        EditSelectTrace::render_select_box(state.clone(), trace, index, &resize_info)
                    }))
            }));

        html!("empty-fragment", {
            .child(
                svg::render_masks(
                    mask_children,
                    draw_children,
                    clone!(state => move |x, y| {
                        TracesEdit::start_draw(state.clone(), None, Some((x, y)));
                    }),
                    |_x, _y| { },
                    |_x, _y| { },
                )
            )
            .children_signal_vec(menu_children)
        })
    }
}

fn render_trace<S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &EditSelectTrace,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    S: Signal<Item = ShapeStyle> + 'static,
{
    let trace_size = trace.size;

    let transform_size = Some(TransformSize::Dynamic(
        trace
            .select_box
            .transform_override
            .signal_cloned()
            .map(move |t| (t, trace_size)),
    ));
    match trace.shape {
        TraceShape::PathCommands(ref commands) => svg::render_path_commands(
            shape_style,
            resize_info,
            transform_size,
            commands,
            callbacks,
        ),

        TraceShape::Path(ref path) => {
            svg::render_path(shape_style, resize_info, transform_size, path, callbacks)
        }

        TraceShape::Rect(width, height) => svg::render_rect(
            shape_style,
            resize_info,
            transform_size,
            width,
            height,
            callbacks,
        ),

        TraceShape::Ellipse(radius_x, radius_y) => svg::render_ellipse(
            shape_style,
            resize_info,
            transform_size,
            radius_x,
            radius_y,
            callbacks,
        ),
    }
}
