use futures_signals::signal::ReadOnlyMutable;

use crate::stickers::{
    embed::state::Embed,
    state::{Sticker, Stickers},
};
use std::rc::Rc;

pub struct State {
    pub stickers: Rc<Stickers<Sticker>>,
    pub embed: ReadOnlyMutable<Option<Rc<Embed>>>,
}

impl State {
    pub fn new(
        stickers: &Rc<Stickers<Sticker>>,
        embed: ReadOnlyMutable<Option<Rc<Embed>>>,
    ) -> Self {
        Self {
            stickers: Rc::clone(&stickers),
            embed,
        }
    }
}
