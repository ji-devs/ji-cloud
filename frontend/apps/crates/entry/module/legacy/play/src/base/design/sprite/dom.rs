use crate::base::{design::sprite::{SpriteData, player::SpritePlayer}, state::Base};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite
};
use std::{borrow::Borrow, rc::Rc};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::canvas::{get_2d_context, CanvasToBlobFuture};
use super::state::{Sprite, SpritePhase};

impl Sprite {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self.clone();

        html!("empty-fragment", {
            .future(clone!(state => async move {

                let url = state.base.media_url(&state.raw.src);

                //TODO - load GIF if animation
                let img = match awsm_web::loaders::image::load(url).await {
                    Ok(img) => img,
                    Err(_) => {
                        panic!("could not load image!");
                    }
                };

                state.size.set(Some((
                    img.natural_width() as f64,
                    img.natural_height() as f64,
                )));

                state.data.set(Some(
                    SpriteData::Static(img)
                ));

            }))
            .child_signal(state.data.signal_cloned().map(clone!(state => move |data| data.map(|data| {
                let transform_matrix = Matrix4::new_direct(self.raw.transform_matrix.clone());
                let transform_signal = resize_info_signal().map(move |resize_info| {
                    let mut m = transform_matrix.clone();
                    m.denormalize(&resize_info);
                    m.as_matrix_string()
                });

                html!("canvas" => web_sys::HtmlCanvasElement, {
                    .future(state.phase.signal_cloned().for_each(clone!(state, data => move |phase| {

                        let ctx = state.ctx.borrow().as_ref().unwrap_ji().clone();

                        *state.player.borrow_mut() = Some(SpritePlayer::new(ctx, data.clone(), phase.clone()));

                        async {}
                    })))
                    .event(clone!(state => move |evt:events::Click| {
                        log::info!("click")
                    }))
                    .style("cursor", "pointer")
                    .style("display", "block")
                    .style("position", "absolute")
                    .style_signal("width", width_signal(state.size.signal_cloned()))
                    .style_signal("height", height_signal(state.size.signal_cloned()))
                    .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
                    .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
                    .style_signal("transform", transform_signal)

                    .after_inserted(clone!(state => move |canvas| {
                        let (natural_width, natural_height) = state.size.get_cloned().unwrap_ji();

                        canvas.set_width(natural_width as u32);
                        canvas.set_height(natural_height as u32);
                        *state.ctx.borrow_mut() = Some(get_2d_context(&canvas, None).unwrap_ji());

                        // todo - set based on settings
                        state.phase.set_neq(SpritePhase::PlayStatic);
                      
                    }))
                })
            }))))
        })
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
