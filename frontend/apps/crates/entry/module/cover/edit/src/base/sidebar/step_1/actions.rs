use crate::base::state::*;
use shared::domain::jig::module::body::ThemeId;

impl Base {
    pub fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set_neq(theme);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.base.theme = theme;
            }
        });
    }
}
