use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use futures_signals::signal::ReadOnlyMutable;

use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Text,
};

pub fn render_sticker_text_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    text: Rc<Text>,
) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        Stickers::duplicate(stickers.clone(), index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
