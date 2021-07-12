use dominator::{html, Dom, clone, DomBuilder, apply_methods};
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
    super::state::{Stickers, AsSticker},
    actions::load_and_render,
    menu::dom::render_sticker_sprite_menu
};
use web_sys::HtmlElement;
use crate::transform::dom::render_transform;
use shared::domain::jig::module::body::{_groups::design::Sprite as RawSprite, Transform};
//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

pub fn render_sticker_sprite<T: AsSticker>(stickers:Rc<Stickers<T>>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {
    html!("empty-fragment", {
        .child(
            html!("empty-fragment", {
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
            })
        )
        .child_signal(stickers.selected_signal(index.clone()).map(clone!(stickers, sprite, index => move |active| {
            if active {
                Some(render_transform(
                    sprite.transform.clone(),
                    true,
                    Some(clone!(stickers, index, sprite => move || render_sticker_sprite_menu(stickers.clone(), index.clone(), sprite.clone())))
                ))
            } else {
                None
            }
        })))
    })

}

pub fn render_sticker_sprite_raw(sprite: &RawSprite) -> Dom {
    _render_sticker_sprite_raw_mixin(sprite, None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>)
}

pub fn render_sticker_sprite_raw_mixin<F>(sprite: &RawSprite, mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    _render_sticker_sprite_raw_mixin(sprite, Some(mixin))
}

fn _render_sticker_sprite_raw_mixin<F>(sprite: &RawSprite, mixin: Option<F>) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{

    _render_sticker_sprite_raw_parent_mixin(DomBuilder::new_html("empty-fragment"), sprite, mixin)

}

pub fn render_sticker_sprite_raw_parent(parent: DomBuilder<HtmlElement>, sprite: &RawSprite) -> Dom
{
    _render_sticker_sprite_raw_parent_mixin(parent, sprite, None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>) 
}

pub fn render_sticker_sprite_raw_parent_mixin<F>( parent: DomBuilder<HtmlElement>, sprite: &RawSprite,child_mixin: F) -> Dom 
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    _render_sticker_sprite_raw_parent_mixin(parent, sprite, Some(child_mixin))
}

//Yeah it's a bit weird, but helpful for creating generic containers like StickerOutline
//The idea is that the sticker sets styles on the parent and then appends itself
//So the parent gets transformed etc.
fn _render_sticker_sprite_raw_parent_mixin<F>(parent: DomBuilder<HtmlElement>, sprite: &RawSprite, child_mixin: Option<F>) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
{
    let src:Mutable<Option<String>> = Mutable::new(None);
    let size:Mutable<Option<(f64, f64)>> = Mutable::new(None);

    let RawSprite { image, effects, flip_horizontal, flip_vertical, ..} = sprite;

    parent
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
        .apply_if(child_mixin.is_some(), |dom| dom.apply(child_mixin.unwrap_ji()))
        .into_dom()

}
