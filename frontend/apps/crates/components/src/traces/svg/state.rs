use std::rc::Rc;
use dominator::DomBuilder;
use utils::{prelude::*, resize::ResizeInfo};

use futures_signals::signal::Signal;

use shared::domain::jig::module::body::{Transform, _groups::design::TraceKind};

type PlaceholderSignal<T> = futures_signals::signal::Always<T>;
type PlaceholderTransformSizeSignal = PlaceholderSignal<(Transform, (f64, f64))>;
type PlaceholderShapeStyleSignal = PlaceholderSignal<ShapeStyle>;

pub enum ShapeStyleVar<S>
where
    S: Signal<Item = ShapeStyle>,
{
    Static(ShapeStyle),
    Dynamic(S),
}
impl ShapeStyleVar<PlaceholderShapeStyleSignal> {
    pub fn new_static(shape_style: ShapeStyle) -> ShapeStyleVar<PlaceholderShapeStyleSignal> {
        Self::Static(shape_style)
    }

    pub fn none() -> Option<ShapeStyleVar<PlaceholderShapeStyleSignal>> {
        None
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ShapeStyle {
    // whether or not it should show the cursor
    pub interactive: bool,
    pub mode: ShapeStyleMode,
    pub kind: ShapeStyleKind,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ShapeStyleMode {
    Mask,
    Transparent,
    Edit(ShapeStyleEditMode),
    Play(ShapeStylePlayMode),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ShapeStyleEditMode {
    Draw,
    Selected,
    Deselected,
    WithoutCutout,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ShapeStylePlayMode {
    Selected,
    Deselected,
    Hint
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ShapeStyleKind {
    General,
    Correct,
    Incorrect,
}

impl From<TraceKind> for ShapeStyleKind {
    fn from(trace_kind:TraceKind) -> ShapeStyleKind {
        match trace_kind {
            TraceKind::Wrong => Self::Incorrect,
            TraceKind::Correct => Self::Correct,
            TraceKind::Regular => Self::General,
        }
    }
}

impl ShapeStyle {
    pub fn new_mask() -> Self {
        Self {
            interactive: false,
            mode: ShapeStyleMode::Mask,
            kind: ShapeStyleKind::General,
        }
    }

    pub fn new_transparent() -> Self {
        Self {
            interactive: true,
            mode: ShapeStyleMode::Transparent,
            kind: ShapeStyleKind::General,
        }
    }
    pub fn new_edit(mode: ShapeStyleEditMode, kind: ShapeStyleKind) -> Self {
        Self {
            interactive: mode != ShapeStyleEditMode::Draw,
            mode: ShapeStyleMode::Edit(mode),
            kind: kind,
        }
    }

    pub fn new_play(mode: ShapeStylePlayMode, kind: ShapeStyleKind) -> Self {
        Self {
            interactive: mode != ShapeStylePlayMode::Hint,
            mode: ShapeStyleMode::Play(mode),
            kind: kind,
        }
    }
}

pub enum TransformSize<'a, S>
where
    S: Signal<Item = (Transform, (f64, f64))>,
{
    Static(&'a Transform, (f64, f64)),
    Dynamic(S),
}

impl<'a> TransformSize<'a, PlaceholderTransformSizeSignal> {
    pub fn new_static(
        transform: &'a Transform,
        size: (f64, f64),
    ) -> TransformSize<'a, PlaceholderTransformSizeSignal> {
        Self::Static(transform, size)
    }
    pub fn none() -> Option<TransformSize<'a, PlaceholderTransformSizeSignal>> {
        None
    }
}

impl<'a, S> TransformSize<'a, S>
where
    S: Signal<Item = (Transform, (f64, f64))> + 'static,
{
    pub fn get_style_string(
        transform: &Transform,
        size: (f64, f64),
        resize_info: &ResizeInfo,
    ) -> String {
        let (width, height) = resize_info.get_size_px(size.0, size.1);
        format!(
            "transform: {}; transform-origin: {}px {}px;width: {}px; height: {}px;",
            transform.denormalize_matrix_string(resize_info),
            width / 2.0,
            height / 2.0,
            width,
            height
        )
    }
}

pub struct SvgCallbacks {
    pub on_select: Option<Box<dyn Fn()>>,
    pub on_mount: Option<Box<dyn Fn(web_sys::SvgElement)>>,
    pub on_unmount: Option<Box<dyn Fn(web_sys::SvgElement)>>,
    pub mixin: Option<Box<dyn Fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>>,
}
impl SvgCallbacks {
    pub fn new(
        on_select: Option<impl Fn() + 'static>,
        on_mount: Option<impl Fn(web_sys::SvgElement) + 'static>,
        on_unmount: Option<impl Fn(web_sys::SvgElement) + 'static>,
        mixin: Option<impl Fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement> + 'static>,
    ) -> Rc<Self> {
        Rc::new(Self {
            on_select: on_select.map(|f| Box::new(f) as _),
            on_mount: on_mount.map(|f| Box::new(f) as _),
            on_unmount: on_unmount.map(|f| Box::new(f) as _),
            mixin: mixin.map(|f| Box::new(f) as _),
        })
    }

    pub fn select(on_select: impl Fn() + 'static) -> Rc<Self> {
        Self::new(
            Some(on_select),
            None::<fn(web_sys::SvgElement)>,
            None::<fn(web_sys::SvgElement)>,
            None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
        )
    }

    pub fn none() -> Rc<Self> {
        Self::new(
            None::<fn()>,
            None::<fn(web_sys::SvgElement)>,
            None::<fn(web_sys::SvgElement)>,
            None::<fn(DomBuilder<web_sys::SvgElement>) -> DomBuilder<web_sys::SvgElement>>,
        )
    }
}
