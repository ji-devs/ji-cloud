pub mod dom;
pub mod sprite;
pub mod text;
pub mod actions;

use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::Sticker as RawSticker;
use sprite::Sprite;
use text::Text;
use crate::text_editor::state::State as TextEditorState;

pub struct Stickers 
{
    pub list: MutableVec<Sticker>,
    pub selected_index: Mutable<Option<usize>>,
    pub on_change: RefCell<Option<Box<dyn Fn(Vec<RawSticker>)>>>,
}

#[derive(Clone)]
pub enum Sticker {
    /// Sprites
    Sprite(Rc<Sprite>),
    /// Text
    Text(Rc<Text>)
}

impl Stickers {
    pub fn new(raw:&[RawSticker], text_editor: Rc<TextEditorState>, on_change: Option<Box<dyn Fn(Vec<RawSticker>)>>) -> Self {
    
        let list = MutableVec::new_with_values(
                    raw.
                        into_iter()
                        .map(|x| match x {
                            RawSticker::Sprite(sprite) => Sticker::Sprite(Rc::new(Sprite::new(sprite))),
                            RawSticker::Text(text) => Sticker::Text(Rc::new(Text::new(text_editor.clone(), text)))
                        })
                        .collect()
        );

        Self {
            list,
            selected_index: Mutable::new(None),
            on_change: RefCell::new(on_change),
        }
    }

    pub fn get_current(&self) -> Option<Sticker> {
        self
            .selected_index
            .get_cloned()
            .and_then(|i| self.get(i))
    }

    pub fn get_current_as_text(&self) -> Option<Rc<Text>> {
        self
            .get_current()
            .and_then(|sticker| {
                match sticker {
                    Sticker::Text(text) => Some(text.clone()),
                    _ => None
                }
            })
    }

    pub fn get_current_as_sprite(&self) -> Option<Rc<Sprite>> {
        self
            .get_current()
            .and_then(|sticker| {
                match sticker {
                    Sticker::Sprite(sprite) => Some(sprite.clone()),
                    _ => None
                }
            })
    }

    pub fn get(&self, index: usize) -> Option<Sticker> {
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

