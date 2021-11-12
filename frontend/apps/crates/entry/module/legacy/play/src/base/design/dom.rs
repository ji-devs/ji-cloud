use super::styles;
use crate::base::state::Base;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use super::sticker::Sticker;

impl Base {
    pub fn render_design(self: Rc<Self>) -> Dom {
        let state = self;

        html!("empty-fragment", {
            .children(state.slide.design.bgs.iter().map(|src| {
                html!("img", {
                    .class(&*styles::BG)
                    .attribute("src", &state.design_media_url(src))
                })
            }))
            .child(html!("div", {
                .class(&*styles::BG)
                .event(clone!(state => move |_evt:events::Click| {
                    if let Some(cb) = state.bg_click_listener.borrow_mut().as_mut() {
                        cb();
                    }
                }))
            }))
            .children(state.slide.design.stickers
                .iter()
                .enumerate()
                .filter(|(_index, _sticker)| {
                    //*index == 5
                    true
                })
                .map(|(_index, sticker)| {
                    Sticker::new(state.clone(), sticker.clone()).render()
                })
            )
        })
    }
}
