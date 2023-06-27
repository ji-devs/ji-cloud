use components::{
    overlay::handle::OverlayHandle,
    traces::svg::{render_single_shape, ShapeStyle, ShapeStyleVar, SvgCallbacks, TransformSize},
};
use dominator::animation::{Percentage, easing, MutableAnimation};
use dominator::{apply_methods, clone, html, with_node, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use utils::resize::ResizeInfo;
use web_sys::SvgElement;

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

                    fade_animation.animate_to(Percentage::new(0.0));

                    apply_methods!(dom, {
                        .attr_signal("opacity", opacity_signal(fade_animation.clone())) 
                        .with_node!(elem => {
                            .apply(OverlayHandle::lifecycle(clone!(tooltip_text, fade_animation => move || {
                                html!("empty-fragment", {
                                    // the element isn't actually ready to be tracked right away
                                    // but we don't show the tooltip right away either, so all good
                                    .child_signal(tooltip_text.signal_ref(clone!(elem, fade_animation => move |text| {
                                        text.as_ref().map(|text| {

                                            html!("overlay-tooltip-bubble", {
                                                .text(text)
                                                .style("pointer-events", "none")
                                                .style_signal("opacity", opacity_signal(fade_animation.clone())) 
                                                .prop("target", elem.clone())
                                                .prop("targetAnchor", "bm")
                                                .prop("contentAnchor", "oppositeV")
                                                //.prop("strategy", move_strategy.as_str())
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

pub fn opacity_signal(fade_animation: MutableAnimation) -> impl Signal<Item = String> {
    fade_animation
        .signal()
        //TODO support configurable easing
        .map(move |t| easing::in_out(t, easing::cubic))
        .map(|t| 1.0 - t.into_f64())
        .map(|value| format!("{}", value))
}