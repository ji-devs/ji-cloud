use super::styles;
use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::jig::module::body::legacy::design::{
    Sticker
};
use std::rc::Rc;
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::canvas::{get_2d_context, CanvasToBlobFuture};
use super::sprite::Sprite;

impl Base {
    pub fn render_design(self: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .children(self.slide.design.bgs.iter().map(|src| {
                html!("img", {
                    .class(&*styles::BG)
                    .attribute("src", &self.layers_url(src))
                })
            }))
            .children(self.slide.design.stickers.iter().map(|sticker| {
                match sticker {
                    Sticker::Sprite(sprite) => Sprite::new(self.clone(), sprite.clone()).render(),
                    Sticker::Text(_text) => {
                        //TODO
                        html!("empty-text")
                    }
                }
            }))
        })
    }

}