use super::state::PlaySettingsState;
use shared::domain::module::body::cover::Next;

impl PlaySettingsState {
    pub fn set_next(&self, next: Next) {
        self.base.play_settings.next.set(next.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.next = next;
            }
        })
    }
}
