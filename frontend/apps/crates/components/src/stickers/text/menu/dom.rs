use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use futures_signals::signal::{ReadOnlyMutable, SignalExt};

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
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "10px")
        .children(&mut [
            html!("menu-line", {
                .prop("icon", "edit")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        if let Some(text_sticker) = stickers.get_as_text(index) {
                            text_sticker.is_editing.set_neq(true);
                        }
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "duplicate")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.duplicate(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-to-front")
                .event(clone!(stickers, index, text => move |_ :events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_to_front(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-forward")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-backward")
                .event(clone!(stickers, index, text => move |_evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .prop("icon", "move-to-back")
                .event(clone!(stickers, index, text => move |_ :events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_to_back(index);
                    }
                }))
            }),
        ])
        .child_signal(index.signal_cloned().map(clone!(stickers, index, text => move |sticker_index| {
            let can_delete = match sticker_index {
                Some(sticker_index) => match stickers.get_as_text(sticker_index) {
                    Some(sticker) => sticker.can_delete.get(),
                    None => true,
                }
                None => true,
            };

            if can_delete {
                Some(html!("menu-line", {
                    .prop("icon", "delete")
                    .event(clone!(stickers, index, text => move |_evt:events::Click| {
                        text.transform.close_menu();
                        if let Some(index) = index.get() {
                            stickers.delete_index(index);
                        }
                    }))
                }))
            } else {
                None
            }
        })))
    })
}
