use dominator::{html, Dom, clone, svg, class};
use web_sys::AudioContext;
use std::rc::Rc;
use utils::{prelude::*, resize::{resize_info_signal, ResizeInfo}, math::bounds::BoundsF64};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{SignalVec, SignalVecExt},
};
use super::state::*;
use crate::audio_player::AudioPlayer;

pub fn render(state: Rc<TraceBubble>, ctx: &AudioContext) -> Dom {

    //in rem
    let width = 200.0;
    let height = 100.0;

    *state.audio_player_instance.borrow_mut() = state.audio.as_ref().map(|audio| {
        AudioPlayer::play_oneshot(ctx, audio.clone())
    });

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
}
