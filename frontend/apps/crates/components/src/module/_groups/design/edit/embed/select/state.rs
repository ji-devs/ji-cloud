use std::rc::Rc;

use futures_signals::signal::Mutable;

use crate::stickers::{
    embed::{state::Embed, types::PartialEmbedHost},
    state::{Sticker, Stickers},
};

pub struct EmbedSelect {
    pub stickers: Rc<Stickers<Sticker>>,
    pub embed: Mutable<Option<Rc<Embed>>>,
    pub host: Mutable<Option<PartialEmbedHost>>,
}

impl EmbedSelect {
    pub fn new(stickers: &Rc<Stickers<Sticker>>, embed: Mutable<Option<Rc<Embed>>>) -> Rc<Self> {
        let host = embed
            .lock_ref()
            .as_ref()
            .map(|host| host.host.lock_ref().partial());

        Rc::new(Self {
            stickers: Rc::clone(&stickers),
            embed,
            host: Mutable::new(host),
        })
    }
}
