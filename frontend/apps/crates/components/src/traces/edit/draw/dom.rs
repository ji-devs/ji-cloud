use super::{menu::dom::render_draw_menu, state::*, trace::state::*};
use crate::{
    traces::{
        edit::{select::trace::state::*, state::*},
        svg::{self, ShapeStyle, ShapeStyleState, ShapeStyleVar, SvgCallbacks, TransformSize},
    },
    transform::state::ResizeLevel,
};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::MutableVecLockRef,
};
use shared::domain::jig::module::body::_groups::design::TraceShape as RawTraceShape;
use std::rc::Rc;
use utils::resize::{resize_info_signal, ResizeInfo};

use crate::transform::dom::render_transform;

impl TracesEdit {
    pub fn render_draw(parent: Rc<Self>, state: Rc<Draw>) -> Dom {
        let full_list: MutableVecLockRef<Rc<EditSelectTrace>> = parent.list.lock_ref();

        let shadow_traces: Vec<Rc<EditSelectTrace>> = full_list
            .iter()
            .enumerate()
            .filter(|(idx, _value)| Some(*idx) != state.init_index)
            .map(|(_, value)| value.clone())
            .collect();

        let trace_signal = || {
            map_ref! {
                let resize_info = resize_info_signal(),
                let shape = state.trace.shape.signal_cloned(),
                let draw_points = state.draw_points.signal_cloned(),
                let transform = state.trace.transform.get_inner_signal_cloned(),
                let size = state.trace.transform.size.signal_cloned(),
                let display_trace = state.display_trace.signal()
                    => {
                        (resize_info.clone(), size.clone(), display_trace.clone(), draw_points.clone(), shape.clone(), transform.clone())
                    }
            }
        };

        let mask_children = 
            trace_signal().map(clone!(shadow_traces => move |(resize_info, size, display_trace, draw_points, shape, transform)| {

                let mut elements:Vec<Dom> = Vec::new();
                elements.push({
                    let shape_style = ShapeStyleVar::new_static(ShapeStyle::new_mask());

                    if !display_trace {
                        svg::render_path(shape_style, &resize_info, TransformSize::none(), &draw_points, SvgCallbacks::none())
                    } else {
                        let transform_size = size.map(|size| TransformSize::new_static(&transform, size));
                        match shape {

                            TraceShape::PathCommands(commands) => {
                                svg::render_path_commands_signal(shape_style, resize_info.clone(), transform_size, &commands)
                            },
                            TraceShape::Path(path) => {
                                svg::render_path_signal(shape_style, resize_info.clone(), transform_size, &path)
                            },

                            TraceShape::Rect(width, height) => {
                                svg::render_rect(shape_style, &resize_info, transform_size, width, height,SvgCallbacks::none())
                            }
                            TraceShape::Ellipse(radius_x, radius_y) => {
                                svg::render_ellipse(shape_style, &resize_info, transform_size, radius_x, radius_y,SvgCallbacks::none())
                            }
                        }
                    }
                });

                for trace in shadow_traces.iter() {
                    let shape_style = ShapeStyleVar::new_static(ShapeStyle::new_mask());
                    elements.push(render_trace(shape_style, &resize_info, trace,SvgCallbacks::none()))
                }

                elements
            }))
            .to_signal_vec();

        let draw_kind = state.default_kind;

        let draw_children = 
            trace_signal().map(clone!(draw_kind, shadow_traces => move |(resize_info, size, display_trace, draw_points, shape, transform)| {
                let mut elements = shadow_traces
                    .iter()
                    .map(|trace| {
                        let shape_style = ShapeStyleVar::new_static(
                            ShapeStyle {
                                interactive: false,
                                mode: None,
                                kind: Some(trace.kind),
                                state: Some(ShapeStyleState::Deselected)
                            }
                        );

                        render_trace(
                            shape_style,
                            &resize_info,
                            trace,
                            SvgCallbacks::none(),
                        )
                    })
                    .collect::<Vec<Dom>>();

                elements.push({
                    let shape_style = ShapeStyleVar::new_static(
                        ShapeStyle {
                            interactive: false,
                            mode: None,
                            kind: Some(draw_kind),
                            state: Some(ShapeStyleState::Drawing)
                        }
                    );

                    if !display_trace {
                        svg::render_path(shape_style, &resize_info, TransformSize::none(), &draw_points, SvgCallbacks::none())
                    } else {
                        let transform_size = size.map(|size| TransformSize::new_static(&transform, size));
                        match shape {

                            TraceShape::PathCommands(commands) => {
                                svg::render_path_commands_signal(shape_style, resize_info.clone(), transform_size, &commands)
                            },
                            TraceShape::Path(path) => {
                                svg::render_path_signal(shape_style, resize_info.clone(), transform_size, &path)
                            },

                            TraceShape::Rect(width, height) => {
                                svg::render_rect(shape_style, &resize_info, transform_size, width, height,SvgCallbacks::none())
                            }
                            TraceShape::Ellipse(radius_x, radius_y) => {
                                svg::render_ellipse(shape_style, &resize_info, transform_size, radius_x, radius_y,SvgCallbacks::none())
                            }
                        }
                    }
                });

                elements
            }))
            .to_signal_vec();

        let menu_signal = map_ref! {
            let resize_info = resize_info_signal(),
            let menu = state.menu.signal_cloned()
                => {
                    (resize_info.clone(), menu.clone())
                }
        };

        html!("empty-fragment", {
            //.child(svg::render_simple(shadow_children))
            .child(
                svg::render_masks(
                    mask_children,
                    draw_children,
                    clone!(state => move |x, y| {
                        state.start_draw(x, y);
                    }),
                    clone!(state => move |x, y| {
                        state.end_draw(x, y);
                    }),
                    clone!(state => move |x, y| {
                        state.move_draw(x, y);
                    }),
                )
            )
            .children_signal_vec(
                menu_signal.map(clone!(state => move |(resize_info, menu)| {
                    let mut children:Vec<Dom> = Vec::new();
                    if let Some(menu) = menu {
                        children.push(render_draw_menu(state.clone(), menu, &resize_info));
                        children.push(render_transform(
                                state.trace.transform.clone(),
                                ResizeLevel::Full,
                                None as Option<Box<dyn Fn() -> Dom>>
                        ));
                    }

                    children
                }))
                .to_signal_vec()
            )

        })
    }
}

//Ignores drag_offset
fn render_trace<S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &EditSelectTrace,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    S: Signal<Item = ShapeStyle> + 'static,
{
    let transform_size = Some(TransformSize::new_static(
        &trace.transform,
        trace.size.clone(),
    ));

    match trace.shape {
        RawTraceShape::PathCommands(ref commands) => {
            svg::render_path_commands(shape_style, &resize_info, transform_size, &commands, callbacks)
        },

        RawTraceShape::Path(ref path) => {
            svg::render_path(shape_style, &resize_info, transform_size, &path, callbacks)
        },

        RawTraceShape::Rect(width, height) => svg::render_rect(
            shape_style,
            &resize_info,
            transform_size,
            width,
            height,
            callbacks,
        ),
        RawTraceShape::Ellipse(radius_x, radius_y) => svg::render_ellipse(
            shape_style,
            &resize_info,
            transform_size,
            radius_x,
            radius_y,
            callbacks,
        ),
    }
}
