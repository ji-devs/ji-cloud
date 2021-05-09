use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use shared::domain::jig::module::body::Renderable as RawRenderable;
use crate::steps::main::{
    stickers::state::Sticker,
    text::state::Text
};

pub struct Renderables {
    pub list: MutableVec<Renderable>,
    pub selected_index: Mutable<Option<usize>>
}

impl Renderables {
    pub fn new(raw:&[RawRenderable]) -> Self {
    
        let list = MutableVec::new_with_values(
                    raw.
                        into_iter()
                        .map(|x| match x {
                            RawRenderable::Sprite(sprite) => Renderable::Sticker(Rc::new(Sticker::new(sprite))),
                            RawRenderable::Text(text) => Renderable::Text(Rc::new(Text::new(text)))
                        })
                        .collect()
        );

        Self {
            list,
            selected_index: Mutable::new(None),
        }
    }

    pub fn get_current(&self) -> Option<Renderable> {
        self
            .selected_index
            .get_cloned()
            .and_then(|i| self.get(i))
    }

    pub fn get_current_as_text(&self) -> Option<Rc<Text>> {
        self
            .get_current()
            .and_then(|renderable| {
                match renderable {
                    Renderable::Text(text) => Some(text.clone()),
                    _ => None
                }
            })
    }

    pub fn get_current_as_sticker(&self) -> Option<Rc<Sticker>> {
        self
            .get_current()
            .and_then(|renderable| {
                match renderable {
                    Renderable::Sticker(sticker) => Some(sticker.clone()),
                    _ => None
                }
            })
    }

    pub fn get(&self, index: usize) -> Option<Renderable> {
        self.list.lock_ref().get(index).map(|x| x.clone())
    }

    pub fn selected_signal(&self, index: ReadOnlyMutable<Option<usize>>) -> impl Signal<Item = bool> {
        map_ref! {
            let index = index.signal(),
            let selected = self.selected_index.signal_cloned()
                => {
                    match (*index, *selected) {
                        (Some(index), Some(selected)) => {
                            index == selected
                        },
                        _ => false
                    }
                }
        }
    }

}

/// Renderables are things that can be rendered 
#[derive(Clone)]
pub enum Renderable {
    /// Sprites
    Sticker(Rc<Sticker>),
    /// Text
    Text(Rc<Text>)
}
