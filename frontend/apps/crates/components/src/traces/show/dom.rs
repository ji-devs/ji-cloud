use crate::traces::{
    svg::{
        self, ShapeStyle, ShapeStyleEditMode, ShapeStyleKind, ShapeStyleMode, ShapeStyleVar,
        SvgCallbacks, TransformSize,
    },
    utils::*,
};
use dominator::{clone, html, Dom, DomBuilder};
use dominator_helpers::signals::EitherSignal;
use futures_signals::{
    signal::{always, Signal, SignalExt},
    signal_vec,
};
use std::rc::Rc;
use utils::resize::{resize_info_signal, ResizeInfo};

use super::state::*;
use shared::domain::jig::module::body::_groups::design::{Trace, TraceShape};

impl TracesShow {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .apply_if(state.on_select.is_none(), |dom| dom.style("pointer-events", "none"))
            .child_signal(resize_info_signal().map(clone!(state => move |resize_info| {
                let mode = state.mode.clone();

                let draw_children = signal_vec::always(
                    state.traces
                    .iter()
                    .enumerate()
                    .map(clone!(resize_info, mode, state => move |(index, trace)| {
                        let style_kind:ShapeStyleKind = trace.kind.into();

                        let shape_style_signal = state.selected_index
                            .signal_cloned()
                            .switch(clone!(state, mode => move |selected_index| {
                                match &mode {
                                    TracesShowMode::Cutout => {
                                        EitherSignal::Left(always(ShapeStyle {
                                            interactive: state.on_select.is_some(),
                                            mode: ShapeStyleMode::Edit(
                                                if Some(index) == selected_index {
                                                    ShapeStyleEditMode::Selected
                                                } else {
                                                    ShapeStyleEditMode::Deselected
                                                }
                                            ),
                                            kind: style_kind,
                                        }))
                                    },

                                    TracesShowMode::Solid=> {
                                        EitherSignal::Left(always(
                                            ShapeStyle {
                                                interactive: state.on_select.is_some(),

                                                mode: ShapeStyleMode::Edit(
                                                    ShapeStyleEditMode::WithoutCutout
                                                ),
                                                kind: style_kind,
                                            })
                                        )
                                    },

                                    TracesShowMode::Hidden => {
                                        EitherSignal::Left(always(
                                            ShapeStyle {
                                                interactive: state.on_select.is_some(),
                                                mode: ShapeStyleMode::Transparent,
                                                kind: style_kind,
                                            }
                                        ))
                                    },

                                    TracesShowMode::HiddenSolidMap(lookup) => {
                                        let interactive = state.on_select.is_some();
                                        EitherSignal::Right(lookup.signal_ref(move |lookup| {

                                            ShapeStyle {
                                                interactive,
                                                mode: {
                                                    if lookup.contains(&index) {
                                                        ShapeStyleMode::Edit(
                                                            ShapeStyleEditMode::WithoutCutout
                                                        )
                                                    } else {
                                                        ShapeStyleMode::Transparent
                                                    }
                                                },
                                                kind: style_kind,
                                            }
                                        }))
                                    }
                                }
                            }));

                    let shape_style = ShapeStyleVar::Dynamic(shape_style_signal);

                    let callbacks = SvgCallbacks::new(
                        Some(clone!(state, index => move || {
                            state.select_index(index);
                        })),
                        None::<fn(web_sys::SvgElement)>,
                        None::<fn(web_sys::SvgElement)>,
                        None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
                    );
                    render_trace(shape_style, &resize_info, trace, callbacks)
                }))
                .collect()
            );

                match mode {
                    TracesShowMode::Cutout => {
                        let mask_children = resize_info_signal()
                            .map(clone!(state=> move |resize_info| {
                                state.traces
                                    .iter()
                                    .map(move |trace| {
                                        let shape_style = ShapeStyleVar::new_static(ShapeStyle::new_mask());

                                        let callbacks = SvgCallbacks::new(
                                            None::<fn()>,
                                            None::<fn(web_sys::SvgElement)>,
                                            None::<fn(web_sys::SvgElement)>,
                                            None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
                                        );
                                        render_trace(shape_style, &resize_info, trace, callbacks)
                                    })
                                    .collect::<Vec<Dom>>()
                            }))
                            .to_signal_vec();

                        Some(svg::render_masks(
                            mask_children,
                            draw_children,
                            |_x, _y| {
                            },
                            |_x, _y| {
                            },
                            |_x, _y| {
                            },
                        ))
                    },
                    _ => {
                        Some(svg::render_simple(
                            draw_children,
                            |_x, _y| {
                            },
                            |_x, _y| {
                            },
                            |_x, _y| {
                            },
                        ))
                    },
                }
            })))
        })
    }
}

fn render_trace<S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &Trace,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    S: Signal<Item = ShapeStyle> + 'static,
{
    let transform_size = trace
        .calc_size(resize_info)
        .map(|size| TransformSize::new_static(&trace.transform, size));

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
