use super::{
    super::super::state::{AsSticker, Stickers},
    super::state::Video,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::ReadOnlyMutable;
use std::rc::Rc;
use utils::prelude::*;

pub fn render_sticker_video_menu<T: AsSticker>(
    stickers: Rc<Stickers<T>>,
    index: ReadOnlyMutable<Option<usize>>,
    video: Rc<Video>,
) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, index, video => move |_ :events::Click| {
                    video.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, index, video => move |_ :events::Click| {
                    video.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, video => move |_ :events::Click| {
                    video.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
