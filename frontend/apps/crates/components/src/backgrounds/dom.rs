use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
use crate::color_select::actions::rgba8_to_hex;

use super::state::*;
use shared::domain::jig::module::body::{Background, Backgrounds as RawBackgrounds};
#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
}

pub fn render(bg:Rc<Backgrounds>, debug_opts: Option<DebugOptions>) -> Dom {
    let debug_opts = debug_opts.unwrap_or_default();

    let children = map_ref!{
        let layer_1 = bg.layer_1.signal_cloned(),
        let layer_2 = bg.layer_2.signal_cloned()
            => {
                let mut children:Vec<Dom> = Vec::new();
                if let Some(layer_1) = layer_1 {
                    children.push(render_bg(layer_1));
                }
                if let Some(layer_2) = layer_2 {
                    children.push(render_bg(layer_2));
                }

                children
            }
    }
    .to_signal_vec();

    html!("empty-fragment", {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style_signal("width", resize_info_signal().map(|resize_info| {
            format!("{}px", resize_info.width)
        }))
        .style_signal("height", resize_info_signal().map(|resize_info| {
            format!("{}px", resize_info.height)
        }))
        .children_signal_vec(children)

    })
}

pub fn render_raw(bg:&RawBackgrounds) -> Dom {

    let mut children:Vec<Dom> = Vec::new();
    if let Some(layer_1) = bg.layer_1.as_ref() {
        children.push(render_bg(layer_1));
    }
    if let Some(layer_2) = bg.layer_2.as_ref() {
        children.push(render_bg(layer_2));
    }

    html!("empty-fragment", {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style_signal("width", resize_info_signal().map(|resize_info| {
            format!("{}px", resize_info.width)
        }))
        .style_signal("height", resize_info_signal().map(|resize_info| {
            format!("{}px", resize_info.height)
        }))
        .children(children)

    })
}


fn render_bg(bg:&Background) -> Dom {
    match bg {
        Background::Color(color) => {
            html!("div", {
                .style("position", "absolute")
                .style("top", "0")
                .style("left", "0")
                .style("width", "100%")
                .style("height", "100%")
                .style("background-color", rgba8_to_hex(color))
            })
        },
        Background::Image(image) => {
            html!("img-ji", {
                .style("position", "absolute")
                .style("top", "0")
                .style("left", "0")
                .style("display", "block")
                .style("width", "100%")
                .style("height", "100%")
                .property("id", image.id.0.to_string())
                .property("lib", image.lib.to_str())
                .property("size", "full")
            })
        },
        Background::Theme(theme_id) => {
            html!("div", {
                .style("background-color", "red")
            })
        },
    }
}
