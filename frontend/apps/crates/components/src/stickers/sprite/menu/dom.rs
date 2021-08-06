use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Sprite,
};
use futures_signals::signal::ReadOnlyMutable;

pub fn render_sticker_sprite_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    sprite: Rc<Sprite>,
) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        Stickers::duplicate(stickers.clone(), index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, index, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            /* post-beta
            html!("menu-line", {
                .property("icon", "crop")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            */
            html!("menu-line", {
                .property("icon", "remove-white")
                .event(clone!(stickers, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.remove_white();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-horizontal")
                .event(clone!(stickers, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.toggle_flip_horizontal();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-vertical")
                .event(clone!(stickers, sprite => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.toggle_flip_vertical();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, index => move |_evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
