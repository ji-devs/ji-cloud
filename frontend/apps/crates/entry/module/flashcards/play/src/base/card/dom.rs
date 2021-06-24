use utils::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt
};
use crate::base::{
    state::*,
    card::state::*
};
use components::{
    image::element::ImageJi,
    module::_groups::cards::lookup
};

use shared::domain::jig::module::body::_groups::cards::Mode;

pub fn render_media(card:&CardState, mode: Mode, theme_id: ThemeId) -> Dom {
    match &card.media {
        Media::Text(s) => {
            html!("card-text", {
                .property("value", s)
                .property("fontSize", {
                    let font_size = lookup::get_card_font_size(s.len(), theme_id, mode);
                    format!("{}rem", font_size)
                })
            })
        },
        Media::Image(image) => {
            ImageJi::render(&image.id, image.lib, None)
        },
    }
}
