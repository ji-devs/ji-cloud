use dominator::{apply_methods, clone, html, with_node, Dom, DomBuilder};
use utils::resize::ResizeInfo;
use web_sys::SvgElement;

use components::{
    overlay::handle::OverlayHandle,
    traces::svg::{render_single_shape, ShapeStyle, ShapeStyleVar, SvgCallbacks, TransformSize},
};
use futures_signals::signal::Signal;

use super::state::*;

impl Hotspot {
    pub fn render(
        &self,
        resize_info: &ResizeInfo,
        on_selected: impl Fn() + 'static,
        shape_style_signal: impl Signal<Item = ShapeStyle> + 'static,
    ) -> Dom {
        let shape_style = ShapeStyleVar::Dynamic(shape_style_signal);

        // this depends on calc_bounds() being implemented for PathCommands
        if self.raw.transform_matrix.is_some() {
            log::error!("shape has a transform matrix, but no size")
        }

        let tooltip_text = self.tooltip_text.clone();

        render_single_shape(
            shape_style,
            resize_info,
            &self.raw.shape,
            TransformSize::none(),
            SvgCallbacks::new(
                Some(on_selected),
                None::<fn(web_sys::SvgElement)>,
                None::<fn(web_sys::SvgElement)>,
                Some(move |dom: DomBuilder<SvgElement>| {
                    apply_methods!(dom, {
                        .with_node!(elem => {
                            .apply(OverlayHandle::lifecycle(clone!(tooltip_text => move || {
                                html!("empty-fragment", {
                                    // the element isn't actually ready to be tracked right away
                                    // but we don't show the tooltip right away either, so all good
                                    .child_signal(tooltip_text.signal_ref(clone!(elem => move |text| {
                                        text.as_ref().map(|text| {
                                            html!("overlay-tooltip-bubble", {
                                                .text(text)
                                                .property("target", elem.clone())
                                                .property("targetAnchor", "bm")
                                                .property("contentAnchor", "oppositeV")
                                                //.property("strategy", move_strategy.as_str())
                                            })
                                        })
                                    })))
                                })
                            })))
                        })
                    })
                }),
            ),
        )
    }
}
