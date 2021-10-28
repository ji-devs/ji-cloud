use dominator::{html, Dom, clone, svg, DomBuilder, with_node, apply_methods};
use utils::{prelude::*, resize::{ResizeInfo, resize_info_signal}};
use web_sys::SvgElement;
use std::rc::Rc;
use futures_signals::signal::{Signal, SignalExt};
use components::{
    overlay::handle::OverlayHandle,
    traces::svg::{ShapeStyle, ShapeStyleMode, ShapeStyleKind, ShapeStyleVar, SvgCallbacks, TransformSize, render_single_shape}
};
use shared::domain::jig::module::body::_groups::design::TraceKind;
use super::state::*;

impl Hotspot {
    pub fn render(
        &self, 
        resize_info: &ResizeInfo, 
        on_selected: impl Fn() + 'static,
        shape_style_signal: impl Signal<Item = ShapeStyle> + 'static
    ) -> Dom {
        let shape_style = ShapeStyleVar::Dynamic(shape_style_signal);

        // this depends on calc_bounds() being implemented for PathCommands
        if self.raw.transform_matrix.is_some() {
            log::error!("shape has a transform matrix, but no size")
        }

        let tooltip_text = self.tooltip_text.clone();

        render_single_shape(
            shape_style, 
            &resize_info, 
            &self.raw.shape, 
            TransformSize::none(), 
            SvgCallbacks::new(
                Some(on_selected),
                None::<fn(web_sys::SvgElement)>,
                None::<fn(web_sys::SvgElement)>,
                Some(move |dom:DomBuilder<SvgElement>| {
                    apply_methods!(dom, {
                        .with_node!(elem => {
                            .apply(OverlayHandle::lifecycle(clone!(tooltip_text => move || {
                                html!("empty-fragment", {
                                    .child_signal(tooltip_text.signal_ref(clone!(elem => move |text| {
                                        text.as_ref().map(|text| {
                                            html!("overlay-tooltip-bubble", {
                                                .text(text)
                                                .property("target", elem.clone())
                                                .property("targetAnchor", "bm")
                                                .property("contentAnchor", "oppositeV")
                                                //.property("strategy", move_strategy.as_str())
                                            })
                                            // html!("overlay-drag", {
                                            //     .property("target", elem.clone())
                                            //     .property("targetAnchor", "bm")
                                            //     .property("contentAnchor", "tl")
                                            //     .property("marginY", 24.0)
                                            //     .child(html!("div", { 
                                            //         .text(text)
                                            //     }))
                                            // })
                                        })
                                    })))
                                })
                            })))
                        })
                    })
                        // .after_inserted(|elem| {
                        //     log::info!("{:?}", elem);
                        // })
                })
            )
        )
    }
}