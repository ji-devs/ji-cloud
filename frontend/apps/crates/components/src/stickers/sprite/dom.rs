use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{prelude::*, math::{bounds, transform_signals}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{
    state::{Sprite, width_signal, height_signal},
    super::state::Stickers,
    actions::load_and_render,
};
use crate::transform;
use shared::domain::jig::module::body::{Sprite as RawSprite, Transform};
//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {

    html!("empty-fragment", {
        .child(html!("empty-fragment", {

            .style("cursor", "pointer")
            .style_signal("width", sprite.width_signal())
            .style_signal("height", sprite.height_signal())
            .style_signal("transform", sprite.transform.denormalize_matrix_string_signal())
            // And pin the coordinate system to the center regardless of screen size
            .style_signal("top", bounds::size_height_center_rem_signal(sprite.transform.size.signal()))
            .style_signal("left", bounds::size_width_center_rem_signal(sprite.transform.size.signal()))
            .style("display", "block")
            .style("position", "absolute")
            .future(sprite.effects.signal_cloned().for_each(clone!(sprite => move |effects| {
                clone!(sprite => async move {

                    let (src, width, height) = load_and_render(sprite.image.get_cloned(), &effects).await;
                    sprite.transform.size.set(Some((width, height)));
                    sprite.src.set(Some(src));
                })
            })))
            .child_signal(sprite.src.signal_ref(clone!(stickers, index, sprite => move |src| {
                src.as_ref().map(|src| {
                    html!("img", {
                        .attribute("src", src)
                        .style("display", "block")
                        .style("position", "relative")
                        .style_signal("width", sprite.width_signal())
                        .style_signal("height", sprite.height_signal())
                        .style_signal("transform", sprite.inner_transform_signal()) 
                        .event(clone!(index, stickers => move |evt:events::Click| {
                            if let Some(index) = index.get_cloned() {
                                stickers.select_index(index);
                            }
                        }))
                    })
                })
            })))
        }))
        .child_signal(stickers.selected_signal(index.clone()).map(clone!(stickers, sprite, index => move |active| {
            if active {
                Some(transform::dom::render(
                    sprite.transform.clone(),
                    Some(clone!(stickers, index, sprite => move || super::menu::dom::render(stickers.clone(), index.clone(), sprite.clone())))
                ))
            } else {
                None
            }
        })))

    })
}


pub fn render_raw(sprite: &RawSprite) -> Dom {

    let src:Mutable<Option<String>> = Mutable::new(None);
    let size:Mutable<Option<(f64, f64)>> = Mutable::new(None);

    let RawSprite { image, effects, flip_horizontal, flip_vertical, ..} = sprite;

    html!("empty-fragment", {

        .style_signal("width", width_signal(size.signal_cloned()))
        .style_signal("height", height_signal(size.signal_cloned()))
        .style_signal("top", bounds::size_height_center_rem_signal(size.signal()))
        .style_signal("left", bounds::size_width_center_rem_signal(size.signal()))
        .style_signal("transform", transform_signals::denormalize_matrix_string(always(sprite.transform.clone())))

        .style("display", "block")
        .style("position", "absolute")
        .future(clone!(src, size, image, effects => async move {
            let (url, width, height) = load_and_render(image, &effects).await;
            size.set(Some((width, height)));
            src.set(Some(url));
        }))
        .child_signal(src.signal_ref(clone!(size, sprite, flip_horizontal, flip_vertical => move |src| {
            src.as_ref().map(|src| {
                html!("img", {
                    .attribute("src", src)
                    .style("pointer-events", "none")
                    .style("display", "block")
                    .style("position", "relative")
                    .style_signal("width", width_signal(size.signal_cloned()))
                    .style_signal("height", height_signal(size.signal_cloned()))
                    .style("transform", {
                        let x = if flip_horizontal { -1 } else { 1 };
                        let y = if flip_vertical { -1 } else { 1 };

                        format!("scaleX({}) scaleY({})", x, y)
                    })
                })
            })
        })))
    })

}
