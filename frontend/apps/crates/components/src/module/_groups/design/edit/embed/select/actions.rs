use crate::stickers::{
    embed::{
        state::Embed,
        types::{EmbedHost, PartialEmbedHost},
    },
    state::{Sticker, Stickers},
};
use futures_signals::signal::Mutable;
use js_sys::Reflect;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use super::EmbedSelect;

impl EmbedSelect {
    pub fn on_embed_value_change(&self) {
        let partial_host = self.host.lock_ref();
        let full_embed: Option<Rc<Embed>> = self.embed.lock_ref().clone();

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
        Stickers::add_embed(self.stickers.clone(), host);
    }

    fn update_embed_sticker(&self, embed: Rc<Embed>, partial_host: &PartialEmbedHost) {
        let _ = update_full_from_partial(&embed.host, &partial_host);
        embed.playing_started.set_neq(false);
        embed.is_playing.set_neq(false);
        self.stickers.call_change();
    }

    fn delete_embed_sticker(&self) {
        let stickers = self.stickers.list.lock_ref();
        let embed_index = stickers
            .iter()
            .position(|sticker| matches!(sticker, Sticker::Embed(_)));
        match embed_index {
            None => log::info!("No embed to delete"),
            Some(embed_index) => {
                // drop stickers so that delete_index can get a mutable reference
                drop(stickers);
                self.stickers.delete_index(embed_index);
            }
        };
    }
}

fn update_full_from_partial(
    full: &Mutable<EmbedHost>,
    partial: &PartialEmbedHost,
) -> anyhow::Result<()> {
    let mut full = full.lock_mut();
    match (&*full, partial) {
        (EmbedHost::Youtube(full), PartialEmbedHost::Youtube(partial)) => {
            full.update_from_partial(&*partial)
        }
        (EmbedHost::GoogleSheet(full), PartialEmbedHost::GoogleSheet(partial)) => {
            full.update_from_partial(&*partial)
        }
        (_, partial) => {
            *full = partial.full()?;
            Ok(())
        }
    }
}

pub fn set_error(elem: &HtmlElement, error: bool) {
    let _ = Reflect::set(
        elem,
        &JsValue::from_str("error"),
        &JsValue::from_bool(error),
    );
}
