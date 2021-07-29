use crate::stickers::video::state::Video;

use super::{
    state::*,
    sprite::{ext::*, state::*},
    text::{ext::*, state::*},
    video::{ext::*, state::*},
};
use dominator::clone;
use std::rc::Rc;
use shared::{
    media::MediaLibrary,
    domain::{
        image::ImageId,
        jig::module::body::{
            Image,
            Transform,
            _groups::design::{Sticker as RawSticker, Text as RawText, Sprite as RawSprite, Video as RawVideo, VideoHost }
        }
    }
};
use utils::prelude::*;

enum Direction {
    Head,
    Tail 
}
impl <T: AsSticker> Stickers<T> {

    pub fn duplicate(_self: Rc<Self>, index: usize) {
        if let Some(mut raw) = _self.get_raw(index) {
            let sticker = _self.map(index, |item| {
                match &mut raw {
                    RawSticker::Sprite(sprite) => sprite.transform.nudge_for_duplicate(),
                    RawSticker::Text(text) => text.transform.nudge_for_duplicate(),
                    RawSticker::Video(video) => video.transform.nudge_for_duplicate(),
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

    fn move_dir(&self, index: usize, dir:Direction) {
        let curr = index.clone();
        let len = self.list.lock_ref().len();
        let target_index = match dir {
            Direction::Head  => {
                if curr > 0 {
                    Some(curr - 1)
                } else {
                    None
                }
            },
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
        }
    }

    pub fn delete_index(&self, index: usize) {
        self.list.lock_mut().remove(index);
        self.call_change();
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }

    pub fn add_sprite(_self: Rc<Self>, image: Image) {
        _self.add_sticker(T::new_from_sticker(Sticker::Sprite(Rc::new(
            Sprite::new(
                &RawSprite::new(image),
                Some(clone!(_self => move |_| {
                    _self.call_change();
                }))
            )
        ))));
    }

    pub fn add_text(_self: Rc<Self>, value: String) {
        _self.add_sticker(T::new_from_sticker(Sticker::Text(Rc::new(
            Text::new(
                _self.text_editor.clone(),
                &RawText::new(value),
                Some(clone!(_self => move |_| {
                    _self.call_change();
                }))
            )
        ))));
    }

    pub fn add_video(_self: Rc<Self>, value: VideoHost) {
        _self.add_sticker(T::new_from_sticker(Sticker::Video(Rc::new(
            Video::new(
                &RawVideo::new(value),
                Some(clone!(_self => move |_| {
                    _self.call_change();
                }))
            )
        ))));
    }

    pub fn add_sticker(&self, sticker: T) {
        {
            let mut list = self.list.lock_mut();
            list.push_cloned(sticker);
            self.selected_index.set_neq(Some(list.len()-1));
        }
        self.call_change();
    }


    pub fn select_index(&self, index:usize) {
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

    pub fn set_current_text_value(&self, value:String) {
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
}
