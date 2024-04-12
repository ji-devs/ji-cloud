use crate::stickers::embed::state::Embed;

use super::{
    embed::types::EmbedExt,
    sprite::{ext::*, state::*},
    state::*,
    text::state::*,
};
use dominator::clone;
use futures_signals::signal_vec::VecDiff;
use shared::domain::module::body::{
    Image,
    _groups::design::{
        Embed as RawEmbed, EmbedHost, Sprite as RawSprite, Sticker as RawSticker, Text as RawText,
    },
};
use std::rc::Rc;
use utils::prelude::*;

enum Direction {
    Head,
    Tail,
}
impl<T: AsSticker> Stickers<T> {
    pub fn duplicate(self: &Rc<Self>, index: usize) {
        let _self: &Rc<Stickers<T>> = self;
        if let Some(mut raw) = _self.get_raw(index) {
            let sticker = _self.map(index, |item| {
                match &mut raw {
                    RawSticker::Sprite(sprite) => sprite.transform.nudge_for_duplicate(),
                    RawSticker::Text(text) => text.transform.nudge_for_duplicate(),
                    RawSticker::Embed(embed) => embed.transform.nudge_for_duplicate(),
                };

                item.duplicate_with_sticker(Sticker::new(_self.clone(), &raw))
            });

            if let Some(sticker) = sticker {
                _self.add_sticker(sticker);
            }
        }
    }
    pub fn move_forward(&self, index: usize) {
        self.move_dir(index, Direction::Tail);
    }
    pub fn move_backward(&self, index: usize) {
        self.move_dir(index, Direction::Head);
    }

    pub fn move_to_back(&self, index: usize) {
        self.list.lock_mut().move_from_to(index, 0);
        self.select_index(0);
        self.call_change();
        self.call_index_change(VecDiff::Move {
            old_index: index,
            new_index: 0,
        });
    }

    pub fn move_to_front(&self, index: usize) {
        let front_index = self.list.lock_ref().len() - 1;
        self.list.lock_mut().move_from_to(index, front_index);
        self.select_index(front_index);
        self.call_change();
        self.call_index_change(VecDiff::Move {
            old_index: index,
            new_index: front_index,
        });
    }

    fn move_dir(&self, index: usize, dir: Direction) {
        let curr = index;
        let len = self.list.lock_ref().len();
        let target_index = match dir {
            Direction::Head => {
                if curr > 0 {
                    Some(curr - 1)
                } else {
                    None
                }
            }
            Direction::Tail => {
                if curr < len - 1 {
                    Some(curr + 1)
                } else {
                    None
                }
            }
        };

        if let Some(target_index) = target_index {
            self.list.lock_mut().move_from_to(curr, target_index);
            self.select_index(target_index);
            self.call_change();
            self.call_index_change(VecDiff::Move {
                old_index: curr,
                new_index: target_index,
            });
        }
    }

    pub fn delete_index(&self, index: usize) {
        self.list.lock_mut().remove(index);
        self.call_change();
        self.call_index_change(VecDiff::RemoveAt { index });
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }

    pub fn add_sprite(self: Rc<Self>, image: Image) {
        let _self = self;
        _self.add_sticker(T::new_from_sticker(Sticker::Sprite(Rc::new(Sprite::new(
            &RawSprite::new(image),
            Some(clone!(_self => move |_| {
                _self.call_change();
            })),
            // Some(clone!(_self => move || {
            //     _self.deselect();
            // })),
        )))));
    }

    pub fn add_text(self: Rc<Self>, value: String) {
        let _self = self;
        _self.add_sticker(T::new_from_sticker(Sticker::Text(Rc::new(Text::new(
            _self.text_editor.clone(),
            &RawText::from_value(value),
            Some(clone!(_self => move |_| {
                _self.call_change();
            })),
            // Some(clone!(_self => move || {
            //     _self.deselect();
            // })),
        )))));
    }

    pub fn add_embed(self: Rc<Self>, embed: EmbedHost) {
        let _self = self;
        _self.add_sticker(T::new_from_sticker(Sticker::Embed(Rc::new(Embed::new(
            &RawEmbed::new(embed),
            Some(clone!(_self => move |_| {
                _self.call_change();
            })),
            None::<fn()>,
        )))));
    }

    pub fn add_sticker(&self, sticker: T) {
        {
            let mut list = self.list.lock_mut();
            list.push_cloned(sticker);
            self.selected_index.set_neq(Some(list.len() - 1));
        }
        self.call_change();
    }

    pub fn select_index(&self, index: usize) {
        self.stop_current_text_editing();
        self.selected_index.set(Some(index));
    }

    pub fn deselect(&self) {
        self.stop_current_text_editing();
        self.selected_index.set(None);
    }

    pub fn replace_current_sprite_src(&self, image: Image) {
        if let Some(sprite) = self.get_current_as_sprite() {
            log::info!("{:?}", image);
            sprite.image.set_neq(image);
            self.call_change();
        }
    }

    pub fn set_current_text_value(&self, value: String) {
        if let Some(text) = self.get_current_as_text() {
            text.set_value(value);
            self.call_change();
        }
    }
    pub fn stop_current_text_editing(&self) {
        if let Some(text) = self.get_current_as_text() {
            text.is_editing.set_neq(false);
        }
    }

    // Internal - saving/history is done on the module level
    pub fn call_change(&self) {
        if let Some(on_change) = self.callbacks.on_change.as_ref() {
            on_change(&*self.list.lock_ref());
        }
    }

    pub fn call_index_change(&self, diff: VecDiff<T>) {
        if let Some(on_index_change) = self.callbacks.on_index_change.as_ref() {
            on_index_change(diff);
        }
    }
}
