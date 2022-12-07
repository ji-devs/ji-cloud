use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{AudioEffects, JigPlayerSettings, JigUpdateDraftDataRequest};
use utils::editable_asset::EditableJig;

pub struct State {
    pub jig: Rc<EditableJig>,
    pub active_popup: Mutable<Option<ActiveSettingsPopup>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(jig: &Rc<EditableJig>) -> Self {
        Self {
            jig: Rc::clone(jig),
            active_popup: Mutable::new(None),
            loader: AsyncLoader::new(),
        }
    }

    pub fn get_jig_update_req(&self) -> JigUpdateDraftDataRequest {
        JigUpdateDraftDataRequest {
            audio_background: Some(self.jig.audio_background.get_cloned()),
            theme: Some(self.jig.theme.get_cloned()),
            default_player_settings: Some(self.get_player_settings()),
            audio_effects: Some(self.get_audio_effects()),
            ..Default::default()
        }
    }
    fn get_player_settings(&self) -> JigPlayerSettings {
        JigPlayerSettings {
            direction: self.jig.direction.get(),
            display_score: self.jig.display_score.get(),
            track_assessments: self.jig.track_assessments.get(),
            drag_assist: self.jig.drag_assist.get(),
        }
    }
    fn get_audio_effects(&self) -> AudioEffects {
        AudioEffects {
            feedback_positive: self.jig.feedback_positive.get_cloned(),
            feedback_negative: self.jig.feedback_negative.get_cloned(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActiveSettingsPopup {
    Main,
    Background,
    Feedback(FeedbackTab),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FeedbackTab {
    Positive,
    Negative,
}
