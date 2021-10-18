use super::styles;
use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::module::body::legacy::design::*;
use std::rc::Rc;
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};

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
                    Sticker::Sprite(sprite) => self.render_sprite(&sprite),
                    Sticker::Text(_text) => {
                        //TODO
                        html!("empty-text")
                    }
                }
            }))
        })
    }

    fn render_sprite(&self, sprite: &Sprite) -> Dom {
        let size = Mutable::new(None);

        let transform_matrix = Matrix4::new_direct(sprite.transform_matrix.clone());
        let transform_signal = resize_info_signal().map(move |resize_info| {
            let mut m = transform_matrix.clone();
            m.denormalize(&resize_info);
            m.as_matrix_string()
        });

        html!("img" => web_sys:: HtmlImageElement, {
            .attribute("src", &self.layers_url(&sprite.src))
            .style("pointer-events", "none")
            .style("display", "block")
            .style("position", "absolute")
            .style_signal("width", width_signal(size.signal_cloned()))
            .style_signal("height", height_signal(size.signal_cloned()))
            .style_signal("top", bounds::size_height_center_rem_signal(size.signal()))
            .style_signal("left", bounds::size_width_center_rem_signal(size.signal()))
            .style_signal("transform", transform_signal)
            .with_node!(img => {
                .event(clone!(size => move |_evt:events::Load| {
                    let width = img.natural_width() as f64;
                    let height = img.natural_height() as f64;

                    size.set(Some((width, height)));

                }))
            })
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
