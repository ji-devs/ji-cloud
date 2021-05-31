use super::{
    state::*,
    sprite::{ext::*, state::*},
    text::{ext::*, state::*},
};
use dominator::clone;
use std::rc::Rc;
use shared::{
    media::MediaLibrary,
    domain::{
        image::ImageId,
        jig::module::body::{
            Sticker as RawSticker, 
            Text as RawText, 
            Sprite as RawSprite, 
            Transform
        }
    }
};
impl Stickers {
    pub fn delete_index(&self, index: usize) {
        self.list.lock_mut().remove(index);
        self.call_change();
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }

    pub fn add_sprite(_self: Rc<Self>, id: ImageId, lib: MediaLibrary) {
        _self.add_sticker(Sticker::Sprite(Rc::new(
            Sprite::new(
                &RawSprite::new(id, lib),
                Some(clone!(_self => move |_| {
                    _self.call_change();
                }))
            )
        )));
    }

    pub fn add_text(_self: Rc<Self>, value: String) {
        _self.add_sticker(Sticker::Text(Rc::new(
            Text::new(
                _self.text_editor.clone(),
                &RawText::new(value),
                Some(clone!(_self => move |_| {
                    _self.call_change();
                }))
            )
        )));
    }

    pub fn add_sticker(&self, sticker: Sticker) {
        let mut list = self.list.lock_mut();
        list.push_cloned(sticker);
        self.selected_index.set_neq(Some(list.len()-1));
    }


    pub fn select_index(&self, index:usize) {
        self.stop_current_text_editing();
        self.selected_index.set(Some(index));
    }

    pub fn deselect(&self) {
        self.stop_current_text_editing();
        self.selected_index.set(None);
    }

    pub fn set_current_text_value(&self, value:String) {
        if let Some(text) = self.get_current_as_text() {
            text.set_value(value);
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
            let raw:Vec<RawSticker> = 
                self.list.lock_ref()
                    .iter()
                    .map(|sticker| sticker.to_raw())
                    .collect();

            on_change(raw);
        }
    }
}
