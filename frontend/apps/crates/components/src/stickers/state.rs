use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::Sticker as RawSticker;
use super::{
    sprite::state::Sprite,
    text::state::Text,
    callbacks::Callbacks,
};
use crate::text_editor::state::State as TextEditorState;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;

pub struct Stickers 
{
    pub list: MutableVec<Sticker>,
    pub selected_index: Mutable<Option<usize>>,
    pub text_editor: Rc<TextEditorState>,
    pub(super) callbacks: Callbacks,
}

#[derive(Clone)]
pub enum Sticker {
    /// Sprites
    Sprite(Rc<Sprite>),
    /// Text
    Text(Rc<Text>)
}

impl Sticker {
    pub fn new(stickers: Rc<Stickers>, raw:&RawSticker) -> Self {
        match raw {
            RawSticker::Sprite(sprite) => Self::Sprite(Rc::new(
                Sprite::new(
                    sprite,
                    Some(clone!(stickers => move |_| {
                        stickers.call_change();
                    }))
                )
            )),
            RawSticker::Text(text) => Self::Text(Rc::new(
                    Text::new(
                        stickers.text_editor.clone(), 
                        text,
                        Some(clone!(stickers => move |_| {
                            stickers.call_change();
                        }))
                    )
            ))
        }
    }

    pub fn to_raw(&self) -> RawSticker {
        match self {
            Self::Sprite(sprite) => RawSticker::Sprite(sprite.to_raw()),
            Self::Text(text) => RawSticker::Text(text.to_raw()),
        }
    }
}

impl Stickers {
    pub fn to_raw(&self) -> Vec<RawSticker> {
        self.list
            .lock_ref()
            .iter()
            .map(|sticker| sticker.to_raw())
            .collect()
    }

    pub fn new(raw:Option<&[RawSticker]>, text_editor: Rc<TextEditorState>, callbacks: Callbacks) -> Rc<Self> {
  
        let _self = Rc::new(Self{
            text_editor: text_editor.clone(),
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            callbacks,
        });



        if let Some(raw) = raw {
            _self.list.lock_mut().replace_cloned( 
                        raw.
                            into_iter()
                            .map(|x| Sticker::new(_self.clone(), x))
                            .collect()
            );
        }

        _self

    }

    pub fn get_current(&self) -> Option<Sticker> {
        self
            .selected_index
            .get_cloned()
            .and_then(|i| self.get(i))
    }

    pub fn get_index(&self) -> Option<usize> {
        self.selected_index.get_cloned()
    }

    pub fn get_as_text(&self, index: usize) -> Option<Rc<Text>> {
        self
            .get(index)
            .and_then(|sticker| {
                match sticker {
                    Sticker::Text(text) => Some(text.clone()),
                    _ => None
                }
            })
    }
    pub fn get_as_sprite(&self, index: usize) -> Option<Rc<Sprite>> {
        self
            .get(index)
            .and_then(|sticker| {
                match sticker {
                    Sticker::Sprite(sprite) => Some(sprite.clone()),
                    _ => None
                }
            })
    }

    pub fn get_current_as_text(&self) -> Option<Rc<Text>> {
        self.get_index()
            .and_then(|index| self.get_as_text(index))
    }

    pub fn get_current_as_sprite(&self) -> Option<Rc<Sprite>> {
        self.get_index()
            .and_then(|index| self.get_as_sprite(index))
    }

    pub fn get(&self, index: usize) -> Option<Sticker> {
        self.list.lock_ref().get(index).map(|x| x.clone())
    }
    pub fn get_raw(&self, index: usize) -> Option<RawSticker> {
        self.list.lock_ref().get(index).map(|x| x.to_raw())
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

