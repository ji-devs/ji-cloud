use super::state::*;
use dominator::clone;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;

impl TalkType {
    pub fn on_start(self: Rc<Self>) {
        let state = self;

        state.base.allow_stage_click();
    }
}


impl TalkTypeItem {
    pub fn evaluate(self: Rc<Self>, parent: Rc<TalkType>) {

        let state = self;
        let expected = match state.raw.texts.as_ref() {
            Some(x) => &x[0],
            None => ""
        };
        
        let value = &*state.value.lock_ref();

        if value == expected {
            log::info!("winner!");
        } else {
            let hint_letters = &mut *state.hint_letters.borrow_mut();

            match hint_letters.pop() {
                Some(letter) => {

                },
                None => {
                    log::info!("out of hints!");
                }
            }

            let hint_word = hint_letters.to_string();

            log::info!("hint: {}", hint_word);
        }
    }
}

impl HintLetters {
    fn pop(&mut self) -> Option<&str> {
        match self.indices.pop() {
            None => None,
            Some(index) => {
                let mut entry = &mut self.letters[index];
                entry.revealed = true;
                Some(&entry.letter)
            }
        }
    }
}