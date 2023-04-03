use crate::base::state::*;
use components::stickers::{
    embed::state::Embed,
    state::{Sticker, Stickers},
};
use js_sys::Reflect;
use shared::domain::module::body::_groups::design::EmbedHost;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

impl Base {
    pub fn on_link_change(&self, host: EmbedHost) {
        let embed = self.get_embed_sticker();

        match embed {
            None => {
                self.add_embed_sticker(host);
            }
            Some(embed) => {
                self.update_embed_sticker(embed, host);
            }
        }
    }

    fn add_embed_sticker(&self, host: EmbedHost) {
        Stickers::add_embed(Rc::clone(&self.stickers), host);
    }

    fn update_embed_sticker(&self, sticker: Rc<Embed>, host: EmbedHost) {
        sticker.host.set(host);
        sticker.playing_started.set_neq(false);
        sticker.is_playing.set_neq(false);
        Stickers::call_change(&Rc::clone(&self.stickers));
    }

    #[must_use]
    pub fn get_embed_sticker(&self) -> Option<Rc<Embed>> {
        let stickers = self.stickers.list.lock_ref();

        let embed = stickers
            .iter()
            .find(|sticker| matches!(sticker, Sticker::Embed(_)))
            .map(|sticker| match sticker {
                Sticker::Embed(embed) => embed,
                _ => unreachable!("should not be possible"),
            });

        let embed = embed.map(|embed| Rc::clone(&embed));

        embed
    }

    pub fn delete_embed(&self) {
        let mut stickers = self.stickers.list.lock_mut();
        let embed_index = stickers
            .iter()
            .position(|sticker| matches!(sticker, Sticker::Embed(_)));
        match embed_index {
            None => log::info!("No emebd to delete"),
            Some(embed_index) => {
                stickers.remove(embed_index);
            }
        };
    }
}

pub fn set_error(elem: &HtmlElement, error: bool) {
    let _ = Reflect::set(
        elem,
        &JsValue::from_str("error"),
        &JsValue::from_bool(error),
    );
}
