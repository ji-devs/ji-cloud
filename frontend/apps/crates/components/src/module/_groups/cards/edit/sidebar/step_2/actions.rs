use shared::domain::jig::module::body::{Background, ThemeId};

use crate::module::_groups::cards::edit::state::*;

impl<RawData: RawDataExt, E: ExtraExt> CardsBase<RawData, E> {
    pub fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set_neq(theme);

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
