use dominator::{clone, html, Dom, DomBuilder};
use dominator_helpers::signals::DefaultSignal;
use std::rc::Rc;
use utils::{
    math::{bounds, transform_signals},
    prelude::*,
};

use super::{
    super::{
        dom::{BaseRawRenderOptions, BaseRenderOptions},
        state::{AsSticker, Stickers},
    },
    actions::load_and_render,
    menu::dom::render_sticker_sprite_menu,
    state::{height_signal, width_signal, Sprite},
};
use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};

use crate::transform::{dom::render_transform, state::ResizeLevel};
use shared::domain::jig::module::body::_groups::design::Sprite as RawSprite;
//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

#[derive(Default)]
pub struct SpriteRenderOptions {
    pub base: BaseRenderOptions,
}

#[derive(Default)]
pub struct SpriteRawRenderOptions {
    pub base: BaseRawRenderOptions,
}

pub fn render_sticker_sprite<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sprite: Rc<Sprite>,
    opts: Option<SpriteRenderOptions>,
) -> Dom {
    let _opts = opts.unwrap_or_default();

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
                            .event(clone!(index, stickers => move |_evt:events::Click| {
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
                    ResizeLevel::Full,
                    Some(clone!(stickers, index, sprite => move || render_sticker_sprite_menu(stickers.clone(), index.clone(), sprite.clone())))
                ))
            } else {
                None
            }
        })))
    })
}

pub fn render_sticker_sprite_raw(sprite: &RawSprite, opts: Option<SpriteRawRenderOptions>) -> Dom {
    let src: Mutable<Option<String>> = Mutable::new(None);
    let RawSprite {
        image,
        effects,
        flip_horizontal,
        flip_vertical,
        ..
    } = sprite;

    let opts = opts.unwrap_or_default();

    let parent = opts
        .base
        .parent
        .unwrap_or_else(|| DomBuilder::new_html("empty-fragment"));

    let size = opts.base.size.unwrap_or_else(|| Mutable::new(None));

    let transform = sprite.transform.clone();

    let transform_override = opts.base.transform_override;

    let get_transform_signal = clone!(transform, transform_override => move || {
        DefaultSignal::new(
            transform.clone(),
            transform_override.clone().map(clone!(transform => move |t| t.get_signal(transform)))
        )
    });

    let mixin = opts.base.mixin;

    parent
        .style_signal("width", width_signal(size.signal_cloned()))
        .style_signal("height", height_signal(size.signal_cloned()))
        .style_signal("top", bounds::size_height_center_rem_signal(size.signal()))
        .style_signal("left", bounds::size_width_center_rem_signal(size.signal()))
        .style_signal(
            "transform",
            transform_signals::denormalize_matrix_string(get_transform_signal()),
        )
        /*
        .apply_if(!has_parent, |dom| {
            dom.style_signal(
                "transform",
                transform_signals::denormalize_matrix_string(transform_signal),
            )
        })
        */
        .style("display", "block")
        .style("position", "absolute")
        .future(clone!(src, size, image, effects => async move {
            let (url, width, height) = load_and_render(image, &effects).await;
            size.set(Some((width, height)));
            src.set(Some(url));
        }))
        .child_signal(
            src.signal_ref(clone!(size, flip_horizontal, flip_vertical => move |src| {
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
            })),
        )
        .apply_if(mixin.is_some(), move |dom| dom.apply(mixin.unwrap_ji()))
        .into_dom()
}
