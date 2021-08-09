use super::state::State;
use shared::domain::jig::module::body::video::DoneAction;

impl State {
    pub fn toggle_captions(&self) {
        let captions = !*self.base.play_settings.captions.lock_ref();

        self.base.play_settings.captions.set(captions);

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.captions = captions;
            }
        })
    }
    pub fn toggle_muted(&self) {
        let muted = !*self.base.play_settings.muted.lock_ref();

        self.base.play_settings.muted.set(muted);

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.muted = muted;
            }
        });
    }

    pub fn toggle_autoplay(&self) {
        let autoplay = !*self.base.play_settings.autoplay.lock_ref();

        self.base.play_settings.autoplay.set(autoplay);

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.autoplay = autoplay;
            }
        });
    }

    pub fn set_unset_next_action(&self, done_action: Option<DoneAction>) {
        // Set to none if already set to this action
        let done_action = if *self.base.play_settings.done_action.lock_ref() == done_action {
            None
        } else {
            done_action
        };

        self.base.play_settings.done_action.set(done_action);

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.done_action = done_action;
            }
        });
    }
}
