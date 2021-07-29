use std::rc::Rc;
use shared::domain::jig::module::body::{Background, ThemeChoice};
use dominator::clone;
use crate::module::{
    _common::edit::prelude::*,
    _groups::cards::edit::state::*,
};

impl <RawData: RawDataExt, E: ExtraExt> CardsBase<RawData, E> {
    pub fn set_theme(&self, theme: ThemeChoice) { 
        self.theme_choice.set_neq(theme);

        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.theme = theme;
            }
        });
    }

    pub fn set_bg(&self, background: Background) { 
        let bg = Some(background);
        self.background.set(bg.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = raw.get_content_mut() {
                content.background = bg;
            }
        });
    }
}
