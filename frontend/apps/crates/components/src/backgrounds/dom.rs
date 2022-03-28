use dominator::{html, Dom};
use std::rc::Rc;
use utils::{colors::*, prelude::*};

use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};

use super::state::*;
use shared::domain::jig::module::body::{
    Background, _groups::design::Backgrounds as RawBackgrounds,
};

pub fn render_backgrounds(bg: Rc<Backgrounds>, slot: Option<&str>) -> Dom {
    let children = map_ref! {
        let theme_id = bg.theme_id.signal(),
        let layer_1 = bg.layer_1.signal_cloned(),
        let layer_2 = bg.layer_2.signal_cloned()
            => {
                let mut children:Vec<Dom> = Vec::new();

                children.push(render_theme_bg(*theme_id));

                if let Some(layer_1) = layer_1 {
                    if let Some(dom) = render_bg(layer_1) {
                        children.push(dom);
                    }
                }
                if let Some(layer_2) = layer_2 {
                    if let Some(dom) = render_bg(layer_2) {
                        children.push(dom);
                    }
                }

                children
            }
    }
    .to_signal_vec();

    html!("empty-fragment", {
        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
        .style("width", "100%")
        .style("height", "100%")
        .children_signal_vec(children)
    })
}

pub fn render_single_background(
    bg_signal: impl Signal<Item = Option<Background>> + 'static,
    theme_id_signal: impl Signal<Item = ThemeId> + 'static,
    slot: Option<&str>,
) -> Dom {
    let children = map_ref! {
        let theme_id = theme_id_signal,
        let layer = bg_signal
            => {
                let mut children:Vec<Dom> = Vec::new();

                children.push(render_theme_bg(*theme_id));

                if let Some(layer) = layer {
                    if let Some(dom) = render_bg(layer) {
                        children.push(dom);
                    }
                }

                children
            }
    }
    .to_signal_vec();

    html!("empty-fragment", {
        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
        .style("width", "100%")
        .style("height", "100%")
        .children_signal_vec(children)

    })
}
pub fn render_backgrounds_raw(bg: &RawBackgrounds, theme_id: ThemeId, slot: Option<&str>) -> Dom {
    let mut children: Vec<Dom> = Vec::new();

    children.push(render_theme_bg(theme_id));

    if let Some(layer_1) = bg.layer_1.as_ref() {
        if let Some(dom) = render_bg(layer_1) {
            children.push(dom);
        }
    }
    if let Some(layer_2) = bg.layer_2.as_ref() {
        if let Some(dom) = render_bg(layer_2) {
            children.push(dom);
        }
    }

    html!("empty-fragment", {
        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
        .style("width", "100%")
        .style("height", "100%")
        .children(children)

    })
}

pub fn render_single_background_raw(
    bg: &Option<Background>,
    theme_id: ThemeId,
    slot: Option<&str>,
) -> Dom {
    let mut children: Vec<Dom> = Vec::new();

    children.push(render_theme_bg(theme_id));

    if let Some(bg) = bg.as_ref() {
        if let Some(dom) = render_bg(bg) {
            children.push(dom);
        }
    }

    html!("empty-fragment", {
        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
        .style("width", "100%")
        .style("height", "100%")
        .children(children)

    })
}

fn render_bg(bg: &Background) -> Option<Dom> {
    match bg {
        Background::Color(color) => color.map(|color| {
            html!("div", {
                .style("position", "absolute")
                .style("top", "0")
                .style("left", "0")
                .style("width", "100%")
                .style("height", "100%")
                .style("background-color", rgba8_to_hex(&color))
            })
        }),
        Background::Image(image) => Some(html!("img-ji", {
            .style("position", "absolute")
            .style("top", "0")
            .style("left", "0")
            .style("object-fit", "contain")
            .style("width", "100%")
            .style("height", "100%")
            .property("id", image.id.0.to_string())
            .property("lib", image.lib.to_str())
            .property("size", "full")
        })),
    }
}

fn render_theme_bg(theme_id: ThemeId) -> Dom {
    html!("img-ui", {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("display", "block")
        .style("width", "100%")
        .style("height", "100%")
        .property("path", {
            &format!("theme/{}/bg.jpg", theme_id.as_str_id())
        })
    })
}
