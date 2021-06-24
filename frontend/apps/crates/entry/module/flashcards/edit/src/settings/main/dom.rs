use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use components::{
    image::element::ImageJi,
    module::_groups::cards::{
        lookup::{self, Side},
        edit::{
            config,
            state::*
        },
    }
};
use futures_signals::{
    map_ref,
    signal::{SignalExt, ReadOnlyMutable}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card}
};

use utils::prelude::*;

pub fn render(state: Rc<MainSettings>) -> Dom {


    html!("flashcards-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.display_mode
                .signal()
                .map(clone!(state => move |display_mode| {
                    let mut children:Vec<Dom> = Vec::new();
                    children.push(render_card(state.clone(), Side::Left));
                    if display_mode == DisplayMode::Pair {
                        children.push(render_card(state.clone(), Side::Right));
                    }

                    children
                }))
                .to_signal_vec()
        )
    })
}

fn render_card(state: Rc<MainSettings>, side:Side) -> Dom {
    let card = if side == Side::Left { &state.left } else { &state.right };

    let theme_id = state.base.theme_id.get_cloned();
    let mode = state.base.mode.clone();

    html!("play-card", {
        .style("visibility", "visible") 
        .property("size", "flashcards")
        .property("flipOnHover", true)
        .property("flipped", if side == Side::Left { true } else { false }) 
        .property("theme", theme_id.as_str_id())
        .property("mode", mode.as_str_id())
        .property("side", side.as_str_id())
        .child(render_media(&card, mode, theme_id))
    })
}

pub fn render_media(card:&Card, mode: Mode, theme_id: ThemeId) -> Dom {
    match &card {
        Card::Text(s) => {
            html!("card-text", {
                .property("value", s)
                .property("fontSize", {
                    let font_size = lookup::get_card_font_size(s.len(), theme_id, mode);
                    format!("{}rem", font_size)
                })
            })
        },
        Card::Image(image) => {
            match image {
                Some(image) => {
                    ImageJi::render(&image.id, image.lib, None)
                },
                None => {
                    html!("empty-fragment")
                }
            }
        },
    }
}
