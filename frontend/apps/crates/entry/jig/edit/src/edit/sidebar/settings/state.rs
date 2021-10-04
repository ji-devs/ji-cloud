use std::{collections::HashSet, iter::FromIterator};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{
    AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive, JigResponse, JigId,
    JigPlayerSettings, JigUpdateDraftDataRequest, TextDirection,
};
use utils::themes::ThemeId;

pub struct State {
    pub theme: Mutable<ThemeId>,
    pub background_audio: Mutable<Option<AudioBackground>>,
    pub feedback_positive: Mutable<HashSet<AudioFeedbackPositive>>,
    pub feedback_negative: Mutable<HashSet<AudioFeedbackNegative>>,
    pub direction: Mutable<TextDirection>,
    pub display_score: Mutable<bool>,
    pub track_assessments: Mutable<bool>,
    pub drag_assist: Mutable<bool>,
    pub jig_id: JigId,
    pub active_popup: Mutable<Option<ActiveSettingsPopup>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(jig: &JigResponse) -> Self {
        Self {
            theme: Mutable::new(jig.jig_data.theme),
            background_audio: Mutable::new(jig.jig_data.audio_background.clone()),
            feedback_positive: Mutable::new(HashSet::from_iter(
                jig.jig_data.audio_effects.feedback_positive.iter().cloned(),
            )),
            feedback_negative: Mutable::new(HashSet::from_iter(
                jig.jig_data.audio_effects.feedback_negative.iter().cloned(),
            )),
            direction: Mutable::new(jig.jig_data.default_player_settings.direction),
            display_score: Mutable::new(jig.jig_data.default_player_settings.display_score),
            track_assessments: Mutable::new(jig.jig_data.default_player_settings.track_assessments),
            drag_assist: Mutable::new(jig.jig_data.default_player_settings.drag_assist),
            jig_id: jig.id.clone(),
            active_popup: Mutable::new(None),
            loader: AsyncLoader::new(),
        }
    }

    pub fn get_jig_update_req(&self) -> JigUpdateDraftDataRequest {
        JigUpdateDraftDataRequest {
            audio_background: Some(self.background_audio.get_cloned()),
            theme: Some(self.theme.get_cloned()),
            default_player_settings: Some(self.get_player_settings()),
            audio_effects: Some(self.get_audio_effects()),
            ..Default::default()
        }
    }
    fn get_player_settings(&self) -> JigPlayerSettings {
        JigPlayerSettings {
            direction: self.direction.get(),
            display_score: self.display_score.get(),
            track_assessments: self.track_assessments.get(),
            drag_assist: self.drag_assist.get(),
        }
    }
    fn get_audio_effects(&self) -> AudioEffects {
        AudioEffects {
            feedback_positive: self.feedback_positive.get_cloned(),
            feedback_negative: self.feedback_negative.get_cloned(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActiveSettingsPopup {
    Main,
    Theme,
    Background,
    Feedback(FeedbackTab),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FeedbackTab {
    Positive,
    Negative,
}
