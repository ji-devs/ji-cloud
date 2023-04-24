use components::stickers::{
    embed::{
        state::Embed,
        types::{EmbedHost, PartialEmbedHost},
    },
    state::{Sticker, Stickers},
};
use js_sys::Reflect;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use super::state::Step2;

impl Step2 {
    pub fn on_embed_value_change(&self) {
        let partial_host = self.host.lock_ref();
        let full_embed = self.sidebar.base.get_embed_sticker();

        match &*partial_host {
            Some(partial_host) => match full_embed {
                None => {
                    if let Ok(full_host) = partial_host.full() {
                        self.add_embed_sticker(full_host);
                    }
                }
                Some(full_embed) => {
                    self.update_embed_sticker(full_embed, partial_host);
                }
            },
            None => {
                self.delete_embed_sticker();
            }
        }
    }

    pub fn delete_embed(&self) {
        self.delete_embed_sticker();
        self.host.set(None);
    }

    fn add_embed_sticker(&self, host: EmbedHost) {
        let host = (&host).into();
        Stickers::add_embed(self.sidebar.base.stickers.clone(), host);
    }

    fn update_embed_sticker(&self, embed: Rc<Embed>, partial_host: &PartialEmbedHost) {
        let _ = embed.host.lock_ref().update_from_partial(partial_host);
        embed.playing_started.set_neq(false);
        embed.is_playing.set_neq(false);
        self.sidebar.base.stickers.call_change();
    }

    fn delete_embed_sticker(&self) {
        let stickers = self.sidebar.base.stickers.list.lock_ref();
        let embed_index = stickers
            .iter()
            .position(|sticker| matches!(sticker, Sticker::Embed(_)));
        match embed_index {
            None => log::info!("No embed to delete"),
            Some(embed_index) => {
                // drop stickers so that delete_index can get a mutable reference
                drop(stickers);
                self.sidebar.base.stickers.delete_index(embed_index);
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
