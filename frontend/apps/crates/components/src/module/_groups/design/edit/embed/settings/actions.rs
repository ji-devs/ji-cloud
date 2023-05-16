use std::rc::Rc;

use crate::stickers::embed::types::YoutubeEmbed;
use shared::domain::module::body::_groups::design::DoneAction;

use super::state::EmbedSettings;

impl EmbedSettings {
    pub fn toggle_captions(&self, youtube: &Rc<YoutubeEmbed>) {
        youtube.captions.replace_with(|c| !*c);
        self.stickers.call_change();
    }

    pub fn toggle_muted(&self, youtube: &Rc<YoutubeEmbed>) {
        youtube.muted.replace_with(|m| !*m);
        self.stickers.call_change();
    }

    pub fn toggle_autoplay(&self, youtube: &Rc<YoutubeEmbed>) {
        youtube.autoplay.replace_with(|a: &mut bool| !*a);
        self.stickers.call_change();
    }

    pub fn set_unset_next_action(
        &self,
        youtube: &Rc<YoutubeEmbed>,
        done_action: Option<DoneAction>,
    ) {
        // Set to none if already set to this action
        let done_action = if *youtube.done_action.lock_ref() == done_action {
            None
        } else {
            done_action
        };
        youtube.done_action.set(done_action);
        self.stickers.call_change();
    }
}
