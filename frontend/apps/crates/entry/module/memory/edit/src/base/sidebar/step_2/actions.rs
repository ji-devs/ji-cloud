use super::state::*;
use std::rc::Rc;
use shared::domain::jig::module::body::{Background, ThemeChoice};
use crate::base::state::*;
use dominator::clone;

impl Base {
    pub fn set_theme(&self, theme: ThemeChoice) { 
        self.theme_choice.set_neq(theme);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.theme = theme;
            }
        });
    }

    pub fn set_bg(&self, background: Background) { 
        let bg = Some(background);
        self.background.set(bg.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.background = bg;
            }
        });
    }
}
