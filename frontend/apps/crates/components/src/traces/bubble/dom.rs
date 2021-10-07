use dominator::{html, Dom, DomBuilder};

use std::rc::Rc;

use super::state::*;
use crate::audio::mixer::{AUDIO_MIXER, AudioPath, AudioSourceExt};
use crate::tooltip::dom::render_mixin as render_tooltip_mixin;
use web_sys::HtmlElement;
use dominator::clone;

impl TraceBubble {
    pub fn render(state: Rc<Self>) -> Dom {
        //in rem
        let _width = 200.0;
        let _height = 100.0;

        state.reset_ended();
        Self::play_audio(state.clone());
        state.evaluate_all_ended();

        if let Some(tooltip) = state.tooltip.as_ref() {
            render_tooltip_mixin(tooltip.clone(), |dom: DomBuilder<HtmlElement>| {
                state.fade
                    .render(dom)
                    .after_removed(clone!(state => move |elem| {
                        *state.audio_handle.borrow_mut() = None;
                    }))
            })
        } else {
            html!("empty-fragment", {
                .after_removed(clone!(state => move |elem| {
                    *state.audio_handle.borrow_mut() = None;
                }))
            })
        }
        /*
        *state.audio_handle.borrow_mut() = state
            .audio
            .as_ref()
            .map(|audio| AUDIO_MIXER.with(|mixer| mixer.play(audio.as_source(), false)));

        if let Some(tooltip) = state.tooltip.as_ref() {
            render_tooltip_mixin(tooltip.clone(), |dom: DomBuilder<HtmlElement>| {
                state.fade.render(dom)
            })
        } else {
            html!("empty-fragment")
        }
        */
        /*
        //TODO - turn to custom element
        html!("div", {
            .apply(|dom| state.fade.render(dom))
            .style("position", "absolute")
            .style_signal("left", state.bounds.denormalize_signal().map(clone!(width => move |bounds| {
                format!("calc({}px + (({}px - {}rem)/2))", bounds.x, bounds.width, width)
            })))
            .style_signal("top", state.bounds.denormalize_signal().map(clone!(height => move |bounds| {
                format!("calc({}px - {}rem)", bounds.y, height)
            })))
            .style("width", &format!("{}rem", width))
            .style("height", &format!("{}rem", height))
            .style("background-color", "red")
            .style("color", "white")
            .apply_if(state.text.is_some(), |dom| {
                dom.text(&state.text.as_ref().unwrap_ji())
            })
        })
        */
    }
}
