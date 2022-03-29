use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal},
    signal_vec::MutableVec,
};
use utils::prelude::TransformExt;

use std::rc::Rc;

use super::{callbacks::Callbacks, sprite::state::Sprite, text::state::Text, video::state::Video};
use crate::{text_editor::state::State as TextEditorState, transform::state::TransformState};
use dominator::clone;
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;

pub trait AsSticker: AsRef<Sticker> + Clone + 'static {
    fn new_from_sticker(sticker: Sticker) -> Self;
    fn duplicate_with_sticker(&self, sticker: Sticker) -> Self;
}

pub struct Stickers<T: AsSticker> {
    pub list: MutableVec<T>,
    pub selected_index: Mutable<Option<usize>>,
    pub text_editor: Rc<TextEditorState>,
    pub(super) callbacks: Callbacks<T>,
}

#[derive(Clone)]
pub enum Sticker {
    /// Sprites
    Sprite(Rc<Sprite>),
    /// Text
    Text(Rc<Text>),
    /// Text
    Video(Rc<Video>),
}

impl AsRef<Sticker> for Sticker {
    fn as_ref(&self) -> &Sticker {
        self
    }
}
impl AsSticker for Sticker {
    fn new_from_sticker(sticker: Sticker) -> Self {
        sticker
    }
    fn duplicate_with_sticker(&self, sticker: Sticker) -> Self {
        sticker
    }
}

impl Sticker {
    pub fn new<T: AsSticker>(stickers: Rc<Stickers<T>>, raw: &RawSticker) -> Self {
        match raw {
            RawSticker::Sprite(sprite) => Self::Sprite(Rc::new(Sprite::new(
                sprite,
                Some(clone!(stickers => move |_| {
                    stickers.call_change();
                })),
                Some(clone!(stickers => move || {
                    stickers.deselect();
                })),
            ))),
            RawSticker::Text(text) => Self::Text(Rc::new(Text::new(
                stickers.text_editor.clone(),
                text,
                Some(clone!(stickers => move |_| {
                    stickers.call_change();
                })),
                Some(clone!(stickers => move || {
                    stickers.deselect();
                })),
            ))),
            RawSticker::Video(video) => Self::Video(Rc::new(Video::new(
                video,
                Some(clone!(stickers => move |_| {
                    stickers.call_change();
                })),
                None::<fn()>,
            ))),
        }
    }

    pub fn to_raw(&self) -> RawSticker {
        match self {
            Self::Sprite(sprite) => RawSticker::Sprite(sprite.to_raw()),
            Self::Text(text) => RawSticker::Text(text.to_raw()),
            Self::Video(video) => RawSticker::Video(video.to_raw()),
        }
    }

    pub fn get_translation_2d(&self) -> (f64, f64) {
        match self {
            Self::Sprite(sprite) => sprite.transform.get_inner_clone().get_translation_2d(),
            Self::Text(text) => text.transform.get_inner_clone().get_translation_2d(),
            Self::Video(video) => video.transform.get_inner_clone().get_translation_2d(),
        }
    }
    pub fn transform(&self) -> &TransformState {
        match self {
            Self::Sprite(sprite) => &sprite.transform,
            Self::Text(text) => &text.transform,
            Self::Video(video) => &video.transform,
        }
    }
}

impl<T: AsSticker> Stickers<T> {
    pub fn to_raw(&self) -> Vec<RawSticker> {
        self.list
            .lock_ref()
            .iter()
            .map(|sticker| sticker.as_ref().to_raw())
            .collect()
    }

    pub fn new(text_editor: Rc<TextEditorState>, callbacks: Callbacks<T>) -> Rc<Self> {
        Rc::new(Self {
            text_editor,
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            callbacks,
        })
    }

    pub fn replace_all(&self, stickers: Vec<T>) {
        self.list.lock_mut().replace_cloned(stickers);
    }

    pub fn map_current<F, A>(&self, f: F) -> Option<A>
    where
        F: FnOnce(&T) -> A,
    {
        self.selected_index
            .get_cloned()
            .and_then(|i| self.map(i, f))
    }
    pub fn get_current(&self) -> Option<Sticker> {
        self.selected_index.get_cloned().and_then(|i| self.get(i))
    }

    pub fn get_index(&self) -> Option<usize> {
        self.selected_index.get_cloned()
    }

    pub fn get_as_text(&self, index: usize) -> Option<Rc<Text>> {
        self.get(index).and_then(|sticker| match sticker {
            Sticker::Text(text) => Some(text),
            _ => None,
        })
    }
    pub fn get_as_sprite(&self, index: usize) -> Option<Rc<Sprite>> {
        self.get(index).and_then(|sticker| match sticker {
            Sticker::Sprite(sprite) => Some(sprite),
            _ => None,
        })
    }

    pub fn get_current_as_text(&self) -> Option<Rc<Text>> {
        self.get_index().and_then(|index| self.get_as_text(index))
    }

    pub fn get_current_as_sprite(&self) -> Option<Rc<Sprite>> {
        self.get_index().and_then(|index| self.get_as_sprite(index))
    }

    pub fn map<F, A>(&self, index: usize, f: F) -> Option<A>
    where
        F: FnOnce(&T) -> A,
    {
        self.list.lock_ref().get(index).map(|x| f(x))
    }

    pub fn get(&self, index: usize) -> Option<Sticker> {
        self.list.lock_ref().get(index).map(|x| x.as_ref().clone())
    }
    pub fn get_raw(&self, index: usize) -> Option<RawSticker> {
        self.list.lock_ref().get(index).map(|x| x.as_ref().to_raw())
    }

    pub fn selected_signal(
        &self,
        index: ReadOnlyMutable<Option<usize>>,
    ) -> impl Signal<Item = bool> {
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
