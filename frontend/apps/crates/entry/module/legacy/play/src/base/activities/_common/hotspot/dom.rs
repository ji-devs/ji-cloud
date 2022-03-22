use dominator::{apply_methods, clone, html, with_node, Dom, DomBuilder};
use utils::resize::ResizeInfo;
use web_sys::SvgElement;
use dominator::animation::easing;
use components::{
    overlay::handle::OverlayHandle,
    traces::svg::{render_single_shape, ShapeStyle, ShapeStyleVar, SvgCallbacks, TransformSize},
};
use futures_signals::signal::{Signal, SignalExt};
use dominator::animation::Percentage;

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

        let fade_animation = self.fade_animation.clone();
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
                            .apply(OverlayHandle::lifecycle(clone!(tooltip_text, fade_animation => move || {
                                html!("empty-fragment", {
                                    // the element isn't actually ready to be tracked right away
                                    // but we don't show the tooltip right away either, so all good
                                    .child_signal(tooltip_text.signal_ref(clone!(elem, fade_animation => move |text| {
                                        text.as_ref().map(|text| {
                                            
                                            fade_animation.animate_to(Percentage::new(0.0));

                                            let value_signal = fade_animation
                                                .signal()
                                                //TODO support configurable easing
                                                .map(move |t| easing::in_out(t, easing::cubic))
                                                .map(|t| 1.0 - t.into_f64());

                                            html!("overlay-tooltip-bubble", {
                                                .text(text)
                                                .style("pointer-events", "none")
                                                .style_signal("opacity", value_signal.map(|value| format!("{}", value)))
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
