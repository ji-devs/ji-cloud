use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Embed,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::ReadOnlyMutable;
use std::rc::Rc;
use utils::prelude::*;

pub fn render_sticker_embed_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    embed: Rc<Embed>,
) -> Dom {
    html!("div", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "10px")
        .children(&mut [
            html!("menu-line", {
                .prop("icon", "move-to-front")
                .event(clone!(stickers, index, embed => move |_ :events::Click| {
                    embed.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_to_front(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-forward")
                .event(clone!(stickers, index, embed => move |_ :events::Click| {
                    embed.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-backward")
                .event(clone!(stickers, index, embed => move |_ :events::Click| {
                    embed.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-to-back")
                .event(clone!(stickers, index, embed => move |_ :events::Click| {
                    embed.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_to_back(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "delete")
                .event(clone!(stickers, embed => move |_ :events::Click| {
                    embed.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
