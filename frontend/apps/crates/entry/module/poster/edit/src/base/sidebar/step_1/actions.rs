use crate::base::state::*;
use shared::domain::jig::module::body::ThemeChoice;

impl Base {
    pub fn set_theme(&self, theme: ThemeChoice) {
        self.theme_choice.set_neq(theme);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.base.theme = theme;
            }
        });
    }
}
