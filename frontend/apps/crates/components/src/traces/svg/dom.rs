use dominator::{clone, svg, Dom};
use std::rc::Rc;
use utils::{
    prelude::*,
    resize::{resize_info_signal, ResizeInfo},
};

use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::SignalVec,
};
use web_sys::SvgElement;

use super::{super::utils::*, state::*, styles::*};

use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{PathCommand, Trace, TraceShape},
};
use std::fmt::Write;

pub fn render_single_trace<S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &Trace,
    callbacks: Rc<SvgCallbacks>,
) -> Option<Dom>
where
    S: Signal<Item = ShapeStyle> + 'static,
{
    trace.calc_size(resize_info)
        .map(|size| {
            let (width, height) = size;
            let (pos_x, pos_y) = trace.transform.get_denormalized_translation_2d(resize_info);
            let transform = trace.transform.map(|t| {
                let mut t = t.clone();
                t.set_translation_2d(0.0, 0.0);
                t
            });


            //Note - currently can't apply style directly, so need to set it as an attribute
            let styles = format!("position: absolute; left: {}px; top: {}px;", pos_x, pos_y);

            //FIXME: the + 100 is a fudge-factor to account for stroke size
            svg!("svg", {
                .attribute("style", &styles)
                .attribute("width", &format!("{}px", width + 100.0))
                .attribute("height", &format!("{}px", height + 100.0))
                .child(render_single_trace_shape(shape_style, resize_info, trace, Some(TransformSize::new_static(&transform, size)), callbacks))
            })
        })
}

pub fn render_single_shape<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    shape: &TraceShape,
    transform_size: Option<TransformSize<'_, T>>,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    match shape {
        TraceShape::PathCommands(ref commands) => render_path_commands(
            shape_style,
            resize_info,
            transform_size,
            commands,
            callbacks,
        ),
        TraceShape::Path(ref path) => {
            render_path(shape_style, resize_info, transform_size, path, callbacks)
        }

        TraceShape::Rect(width, height) => render_rect(
            shape_style,
            resize_info,
            transform_size,
            *width,
            *height,
            callbacks,
        ),
        TraceShape::Ellipse(radius_x, radius_y) => render_ellipse(
            shape_style,
            resize_info,
            transform_size,
            *radius_x,
            *radius_y,
            callbacks,
        ),
    }
}

pub fn render_single_trace_shape<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    trace: &Trace,
    transform_size: Option<TransformSize<'_, T>>,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    match transform_size {
        Some(transform_size) => render_single_shape(
            shape_style,
            resize_info,
            &trace.shape,
            Some(transform_size),
            callbacks,
        ),
        None => {
            let transform_size = trace
                .calc_size(resize_info)
                .map(|size| TransformSize::new_static(&trace.transform, size));

            render_single_shape(
                shape_style,
                resize_info,
                &trace.shape,
                transform_size,
                callbacks,
            )
        }
    }
}

pub fn render_masks<
    ChildrenMaskSignal,
    ChildrenDrawSignal,
    OnMouseDownFn,
    OnMouseUpFn,
    OnMouseMoveFn,
>(
    children_mask: ChildrenMaskSignal,
    children_draw: ChildrenDrawSignal,
    on_mouse_down: OnMouseDownFn,
    on_mouse_up: OnMouseUpFn,
    on_mouse_move: OnMouseMoveFn,
) -> Dom
where
    ChildrenMaskSignal: SignalVec<Item = Dom> + 'static,
    ChildrenDrawSignal: SignalVec<Item = Dom> + 'static,
    OnMouseDownFn: Fn(i32, i32) + Clone + 'static,
    OnMouseUpFn: Fn(i32, i32) + Clone + 'static,
    OnMouseMoveFn: Fn(i32, i32) + Clone + 'static,
{
    svg!("svg", {
        .class(&*SVG_CLASS)
        .attribute_signal("width", resize_info_signal().map(|info| {
            format!("{}px", info.width)
        }))
        .attribute_signal("height", resize_info_signal().map(|info| {
            format!("{}px", info.height)
        }))
        .child(svg!("rect", {
            .attribute("mask", "url(#maskPath)")
            .attribute("x", "0")
            .attribute("y", "0")
            .attribute_signal("width", resize_info_signal().map(|info| {
                format!("{}px", info.width)
            }))
            .attribute_signal("height", resize_info_signal().map(|info| {
                format!("{}px", info.height)
            }))
            .class(&*BG_FILL_CLASS)
            .event(clone!(on_mouse_down => move |evt:events::MouseDown| {
                on_mouse_down(evt.x() as i32, evt.y() as i32);
            }))
        }))
        .child(svg!("defs", {
            .child(svg!("mask", {
                .attribute("id", "maskPath")
                .child(svg!("rect", {
                    .attribute("x", "0")
                    .attribute("y", "0")
                    .attribute_signal("width", resize_info_signal().map(|info| {
                        format!("{}px", info.width)
                    }))
                    .attribute_signal("height", resize_info_signal().map(|info| {
                        format!("{}px", info.height)
                    }))
                    .class(&*BG_MASK_CLASS)
                }))
                .children_signal_vec(children_mask)
            }))
        }))
        .children_signal_vec(children_draw)

        .global_event(clone!(on_mouse_up => move |evt:events::MouseUp| {
            on_mouse_up(evt.x() as i32, evt.y() as i32);
        }))
        .global_event(clone!(on_mouse_move => move |evt:events::MouseMove| {
            on_mouse_move(evt.x() as i32, evt.y() as i32);
        }))
    })
}

pub fn render_simple<ChildrenSignal, OnMouseDownFn, OnMouseUpFn, OnMouseMoveFn>(
    children: ChildrenSignal,
    on_mouse_down: OnMouseDownFn,
    on_mouse_up: OnMouseUpFn,
    on_mouse_move: OnMouseMoveFn,
) -> Dom
where
    ChildrenSignal: SignalVec<Item = Dom> + 'static,
    OnMouseDownFn: Fn(i32, i32) + Clone + 'static,
    OnMouseUpFn: Fn(i32, i32) + Clone + 'static,
    OnMouseMoveFn: Fn(i32, i32) + Clone + 'static,
{
    svg!("svg", {
        .class(&*SVG_CLASS)
        .attribute_signal("width", resize_info_signal().map(|info| {
            format!("{}px", info.width)
        }))
        .attribute_signal("height", resize_info_signal().map(|info| {
            format!("{}px", info.height)
        }))
        .children_signal_vec(children)
        .event(clone!(on_mouse_down => move |evt:events::MouseDown| {
            on_mouse_down(evt.x() as i32, evt.y() as i32);
        }))

        .global_event(clone!(on_mouse_up => move |evt:events::MouseUp| {
            on_mouse_up(evt.x() as i32, evt.y() as i32);
        }))
        .global_event(clone!(on_mouse_move => move |evt:events::MouseMove| {
            on_mouse_move(evt.x() as i32, evt.y() as i32);
        }))
    })
}

pub fn render_path_signal<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    points: &Mutable<Vec<(f64, f64)>>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let path_string = points.signal_ref(clone!(resize_info => move |points| {
        if points.len() < 2 {
            String::from("M 0 0")
        } else {
            path_to_string(
                points
                    .iter()
                    .map(|(x, y)| {
                        resize_info.get_pos_denormalized(*x, *y)
                    })
            )
        }
    }));

    svg!("path", {
        .apply(|dom| shape_style.apply(dom))
        .attribute_signal("d", path_string)
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, &resize_info)
        })
    })
}

pub fn render_path_commands_signal<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    commands: &Mutable<Vec<(PathCommand, bool)>>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let path_string = commands.signal_ref(clone!(resize_info => move |commands| {
        path_command_to_string(
                commands
                    .iter()
                    .map(|(command, absolute)| {
                        (
                            denormalize_command(command, &resize_info),
                            *absolute
                        )
                    })
            )
    }));

    svg!("path", {
        .apply(|dom| shape_style.apply(dom))
        .attribute_signal("d", path_string)
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, &resize_info)
        })
    })
}
pub fn render_path_commands<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    commands: &[(PathCommand, bool)],
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let path_string = path_command_to_string(
        commands
            .iter()
            .map(|(command, absolute)| (denormalize_command(command, resize_info), *absolute)),
    );

    svg!("path" => SvgElement, {
        .apply(|dom| shape_style.apply(dom))
        .attribute("d", &path_string)
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, resize_info)
        })
        .apply(|dom| SvgCallbacks::mixin(callbacks, dom))
    })
}

pub fn render_path<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    points: &[(f64, f64)],
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let path_string = {
        if points.len() < 2 {
            String::from("M 0 0")
        } else {
            path_to_string(
                points
                    .iter()
                    .map(|(x, y)| resize_info.get_pos_denormalized(*x, *y)),
            )
        }
    };

    svg!("path" => SvgElement, {
        .apply(|dom| shape_style.apply(dom))
        .attribute("d", &path_string)
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, resize_info)
        })
        .apply(|dom| SvgCallbacks::mixin(callbacks, dom))
    })
}

pub fn render_rect<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    width: f64,
    height: f64,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let (width, height) = resize_info.get_pos_denormalized(width, height);

    svg!("rect", {
        .apply(|dom| shape_style.apply(dom))
        .attribute("width", &format!("{}px", width))
        .attribute("height", &format!("{}px", height))
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, resize_info)
        })
        .apply(|dom| SvgCallbacks::mixin(callbacks, dom))
    })
}

pub fn render_ellipse<T, S>(
    shape_style: ShapeStyleVar<S>,
    resize_info: &ResizeInfo,
    transform_size: Option<TransformSize<'_, T>>,
    radius_x: f64,
    radius_y: f64,
    callbacks: Rc<SvgCallbacks>,
) -> Dom
where
    T: Signal<Item = (Transform, (f64, f64))> + 'static,
    S: Signal<Item = ShapeStyle> + 'static,
{
    let (radius_x, radius_y) = resize_info.get_pos_denormalized(radius_x, radius_y);

    svg!("ellipse", {
        .apply(|dom| shape_style.apply(dom))
        .attribute("cx", &format!("{}px", radius_x))
        .attribute("cy", &format!("{}px", radius_y))
        .attribute("rx", &format!("{}px", radius_x))
        .attribute("ry", &format!("{}px", radius_y))
        .apply_if(transform_size.is_some(), |dom| {
            transform_size.unwrap_ji().mixin(dom, resize_info)
        })
        .apply(|dom| SvgCallbacks::mixin(callbacks, dom))
    })
}

fn path_command_to_string(path: impl Iterator<Item = (PathCommand, bool)>) -> String {
    let mut output = String::new();

    for (index, (command, absolute)) in path.enumerate() {
        if index != 0 {
            output.push(' ')
        }
        let prefix = match &command {
            PathCommand::MoveTo(_, _) => {
                if absolute {
                    "M"
                } else {
                    "m"
                }
            }
            PathCommand::ClosePath => {
                if absolute {
                    "Z"
                } else {
                    "z"
                }
            }
            PathCommand::LineTo(_, _) => {
                if absolute {
                    "L"
                } else {
                    "l"
                }
            }
            PathCommand::HorizontalLineTo(_) => {
                if absolute {
                    "H"
                } else {
                    "h"
                }
            }
            PathCommand::VerticalLineTo(_) => {
                if absolute {
                    "V"
                } else {
                    "v"
                }
            }
            PathCommand::CurveTo(_, _, _, _, _, _) => {
                if absolute {
                    "C"
                } else {
                    "c"
                }
            }
            PathCommand::SmoothCurveTo(_, _, _, _) => {
                if absolute {
                    "S"
                } else {
                    "s"
                }
            }
            PathCommand::QuadCurveTo(_, _, _, _) => {
                if absolute {
                    "Q"
                } else {
                    "q"
                }
            }
            PathCommand::SmoothQuadCurveTo(_, _) => {
                if absolute {
                    "T"
                } else {
                    "t"
                }
            }
            PathCommand::ArcTo(_, _, _, _, _, _, _) => {
                if absolute {
                    "A"
                } else {
                    "a"
                }
            }
        };

        output.push_str(prefix);

        match command {
            PathCommand::MoveTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::ClosePath => {}
            PathCommand::LineTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::HorizontalLineTo(x) => {
                write!(&mut output, "{}", x).unwrap_ji();
            }
            PathCommand::VerticalLineTo(y) => {
                write!(&mut output, "{}", y).unwrap_ji();
            }
            PathCommand::CurveTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                write!(
                    &mut output,
                    "{} {} {} {} {} {}",
                    cp1x, cp1y, cp2x, cp2y, x, y
                )
                .unwrap_ji();
            }
            PathCommand::SmoothCurveTo(cp1x, cp1y, x, y) => {
                write!(&mut output, "{} {} {} {}", cp1x, cp1y, x, y).unwrap_ji();
            }
            PathCommand::QuadCurveTo(cp1x, cp1y, x, y) => {
                write!(&mut output, "{} {} {} {}", cp1x, cp1y, x, y).unwrap_ji();
            }
            PathCommand::SmoothQuadCurveTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::ArcTo(a, b, c, d, e, f, g) => {
                write!(&mut output, "{} {} {} {} {} {} {}", a, b, c, d, e, f, g).unwrap_ji();
            }
        }
    }

    output
}

fn path_to_string(path: impl Iterator<Item = (f64, f64)>) -> String {
    let mut output = String::from("M");
    for (_index, (x, y)) in path.enumerate() {
        write!(&mut output, " {} {}", x, y).unwrap_ji();
    }

    output.push_str(" Z");

    output
}
