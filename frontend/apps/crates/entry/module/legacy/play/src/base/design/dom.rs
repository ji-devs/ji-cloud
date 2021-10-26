use super::styles;
use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::rc::Rc;
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::canvas::{get_2d_context, CanvasToBlobFuture};
use super::sticker::Sticker;

impl Base {
    pub fn render_design(self: Rc<Self>) -> Dom {
        html!("empty-fragment", {
            .children(self.slide.design.bgs.iter().map(|src| {
                html!("img", {
                    .class(&*styles::BG)
                    .attribute("src", &self.media_url(src))
                })
            }))
            .children(self.slide.design.stickers
                .iter()
                .enumerate()
                .filter(|(index, sticker)| {
                    //*index == 5 
                    true
                })
                .map(|(index, sticker)| {
                    Sticker::new(self.clone(), sticker.clone()).render()
                })
            )
        })
    }

}