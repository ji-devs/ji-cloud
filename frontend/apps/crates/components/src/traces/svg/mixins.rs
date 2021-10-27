use dominator::{clone, DomBuilder};
use std::rc::Rc;
use utils::{prelude::*, resize::ResizeInfo};

use futures_signals::signal::{Signal, SignalExt};
use web_sys::SvgElement;

use super::state::*;

use shared::domain::jig::module::body::Transform;

impl<S> ShapeStyleVar<S>
where
    S: Signal<Item = ShapeStyle> + 'static,
{
    //must consume self because of the signal
    pub fn apply(self, dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
        match self {
            ShapeStyleVar::Static(shape_style) => shape_style.apply(dom),
            ShapeStyleVar::Dynamic(shape_style_signal) => dom.attr_signal(
                "class",
                shape_style_signal.map(|shape_style| shape_style.classes_string()),
            ),
        }
    }
}

impl ShapeStyle {
    pub fn apply(&self, dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
        self.classes()
            .iter()
            .fold(dom, |dom, class_name| dom.class(class_name))
    }
}

impl<'a, S> TransformSize<'a, S>
where
    S: Signal<Item = (Transform, (f64, f64))> + 'static,
{
    pub fn mixin(
        self,
        dom: DomBuilder<web_sys::SvgElement>,
        resize_info: &ResizeInfo,
    ) -> DomBuilder<web_sys::SvgElement> {
        match self {
            Self::Static(transform, size) => {
                let style = Self::get_style_string(transform, size, resize_info);
                dom.attribute("style", &style)
            }
            Self::Dynamic(s) => dom.attribute_signal(
                "style",
                s.map(clone!(resize_info => move |(transform, size)| {
                    Self::get_style_string(&transform, size, &resize_info)
                })),
            ),
        }
    }
}

fn _apply_transform<A: AsRef<web_sys::Element>>(
    dom: DomBuilder<A>,
    resize_info: &ResizeInfo,
    transform_size: Option<(&Transform, (f64, f64))>,
) -> DomBuilder<A> {
    dom.apply_if(transform_size.is_some(), |dom| {
        let (transform, size) = transform_size.unwrap_ji();
        let (width, height) = resize_info.get_size_px(size.0, size.1);
        let style = format!(
            "transform: {}; transform-origin: {}px {}px;width: {}px; height: {}px;",
            transform.denormalize_matrix_string(resize_info),
            width / 2.0,
            height / 2.0,
            width,
            height
        );

        dom.attribute("style", &style)
    })
}

impl SvgCallbacks {
    pub fn mixin(
        callbacks: Rc<Self>,
        dom: DomBuilder<web_sys::SvgElement>,
    ) -> DomBuilder<web_sys::SvgElement> {
        dom.apply_if(callbacks.on_select.is_some(), |dom| {
            dom.event(clone!(callbacks => move |_evt:events::Click| {
                if let Some(on_select) = &callbacks.on_select {
                    (on_select)();
                }
            }))
        })
        .apply_if(callbacks.on_mount.is_some(), |dom| {
            dom.after_inserted(clone!(callbacks => move |elem| {
                if let Some(on_mount) = &callbacks.on_mount {
                    (on_mount)(elem);
                }
            }))
        })
        .apply_if(callbacks.on_unmount.is_some(), |dom| {
            dom.after_removed(clone!(callbacks => move |elem| {
                if let Some(on_unmount) = &callbacks.on_unmount {
                    (on_unmount)(elem);
                }
            }))
        })
    }
}
