use futures_signals::signal::ReadOnlyMutable;

use crate::stickers::{
    embed::state::Embed,
    state::{Sticker, Stickers},
};
use std::rc::Rc;

pub struct EmbedSettings {
    pub stickers: Rc<Stickers<Sticker>>,
    pub embed: ReadOnlyMutable<Option<Rc<Embed>>>,
}

impl EmbedSettings {
    pub fn new(
        stickers: &Rc<Stickers<Sticker>>,
        embed: ReadOnlyMutable<Option<Rc<Embed>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            stickers: Rc::clone(&stickers),
            embed,
        })
    }
}
