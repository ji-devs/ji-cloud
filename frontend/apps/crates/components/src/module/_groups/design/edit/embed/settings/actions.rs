use std::rc::Rc;

use crate::stickers::embed::types::YoutubeEmbed;
use shared::domain::module::body::_groups::design::DoneAction;

use super::state::State;

impl State {
    pub fn toggle_captions(&self, youtube: &Rc<YoutubeEmbed>) {
        let captions = youtube.captions.get();
        youtube.captions.set(captions);
        self.stickers.call_change();
    }

    pub fn toggle_muted(&self, youtube: &Rc<YoutubeEmbed>) {
        let muted = youtube.muted.get();
        youtube.muted.set(muted);
        self.stickers.call_change();
    }

    pub fn toggle_autoplay(&self, youtube: &Rc<YoutubeEmbed>) {
        let autoplay = youtube.autoplay.get();
        youtube.autoplay.set(autoplay);
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
