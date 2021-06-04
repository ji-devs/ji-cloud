use super::state::*;
use shared::domain::jig::module::body::{ThemeChoice, ThemeId};

impl Step2 {
    pub fn change_theme(&self, theme: ThemeChoice) {
        self.base.theme.set_neq(theme);

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.theme = theme;
            }
        });
    }

}
