use super::state::*; 
use shared::domain::jig::module::body::{Sticker, Text, Sprite, Transform};

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


    pub fn select_index(&self, index:usize) {
        self.selected_index.set(Some(index));
    }

    pub fn set_current_text_value(&self, value:String) {
        if let Some(text) = self.get_current_as_text() {
            text.set_value(value);
        }
    }

    pub fn current_text_blur(&self) {
        if let Some(text) = self.get_current_as_text() {
            text.transform.rect_hidden.set_neq(false);
        }
    }

    pub fn deselect(&self) {
        self.selected_index.set(None);
    }

    // Internal - saving/history is done on the module level
    pub fn call_change(&self) {
        if let Some(on_change) = self.on_change.borrow().as_ref() {
            let raw:Vec<Sticker> = 
                self.list.lock_ref()
                    .iter()
                    .map(|sticker| sticker.to_raw())
                    .collect();

            on_change(raw);
        }
    }
}
