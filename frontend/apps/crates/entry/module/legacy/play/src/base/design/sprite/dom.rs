use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite
};
use std::{borrow::Borrow, rc::Rc, cell::RefCell};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::{canvas::{get_2d_context, CanvasToBlobFuture}, data::ArrayBufferExt};
use super::{AnimationPlayer, ImagePlayer, state::{Sprite}};

// http://localhost:4104/module/legacy/play/debug?game_id=17736&slide_index=0&example=true
impl Sprite {
    pub fn render(self: Self) -> Dom {
        match self {
            Self::Image(state) => state.render(),
            Self::Animation(state) => state.render()
        }
    }
}

impl ImagePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix.clone());
        let transform_signal = resize_info_signal().map(move |resize_info| {
            let mut m = transform_matrix.clone();
            m.denormalize(&resize_info);
            m.as_matrix_string()
        });

        html!("img" => web_sys:: HtmlImageElement, {
            .attribute("src", &state.base.media_url(&state.raw.src))
            .style("cursor", "pointer")
            .style("display", "block")
            .style("position", "absolute")
            .style_signal("width", width_signal(state.size.signal_cloned()))
            .style_signal("height", height_signal(state.size.signal_cloned()))
            .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
            .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
            .style_signal("transform", transform_signal)
            .with_node!(img => {
                .event(clone!(state => move |_evt:events::Load| {
                    let width = img.natural_width() as f64;
                    let height = img.natural_height() as f64;

                    state.size.set(Some((width, height)));

                }))
            })
            .event(clone!(state => move |_evt:events::Click| {
                log::info!("clicked!")
            }))
        })
    }
}

impl AnimationPlayer { 
    pub fn render(self: Rc<Self>) -> Dom {

        html!("empty-fragment", {

        })

        // let state = self;

        // let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix.clone());
        // let transform_signal = resize_info_signal().map(move |resize_info| {
        //     let mut m = transform_matrix.clone();
        //     m.denormalize(&resize_info);
        //     m.as_matrix_string()
        // });


        // html!("video" => web_sys:: HtmlVideoElement, {
        //     .children(&mut[
        //         html!("source", {
        //             .attribute("src", &format!("{}.webm", &state.base.media_url(&state.raw.src)))
        //             .attribute("type", "video/webm; codecs=vp9")
        //         }),
        //         html!("source", {
        //             .attribute("src", &format!("{}.mp4", &state.base.media_url(&state.raw.src)))
        //             .attribute("type", "video/mp4; codecs=hvc1")
        //         }),
        //     ])
        //     .property("autoplay", true)
        //     .property("muted", true)
        //     .property("loop", true)
        //     .property("playsinline", true)
        //     .style("cursor", "pointer")
        //     .style("display", "block")
        //     .style("position", "absolute")
        //     .style_signal("width", width_signal(state.size.signal_cloned()))
        //     .style_signal("height", height_signal(state.size.signal_cloned()))
        //     .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
        //     .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
        //     .style_signal("transform", transform_signal)
        //     .with_node!(video => {
        //         .event(clone!(state => move |_evt:events::LoadedMetadata| {
        //             let width = video.video_width() as f64;
        //             let height = video.video_height() as f64;

        //             state.size.set(Some((width, height)));

        //         }))
        //     })
        //     .event(clone!(state => move |_evt:events::Click| {
        //         log::info!("clicked!")
        //     }))
        // })
    }
}
fn width_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.0),
    })
}

fn height_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.1),
    })
}
