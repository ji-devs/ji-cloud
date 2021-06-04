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
use components::image::element::ImageJi;
use shared::domain::jig::module::body::{
    memory::Mode
};

pub fn render_media(card:&CardState, mode: Mode, theme_id: ThemeId) -> Dom {
    match &card.media {
        Media::Text(s) => {
            html!("card-text", {
                .property("value", s)
                .property("fontSize", {
                    let font_size = app_memory_common::lookup::get_card_font_size(s.len(), theme_id);
                    format!("{}rem", font_size)
                })
                .property("fontFamily", {
                    let font_family = app_memory_common::lookup::get_card_font_family(theme_id, mode, card.side.into());
                    theme_id.css_var_font_family(font_family)
                })
                .property("color", { 
                    theme_id.css_var_color(1)
                })
                
            })
        },
        Media::Image(image) => {
            ImageJi::render(&image.id, image.lib, None)
        },
    }
}
