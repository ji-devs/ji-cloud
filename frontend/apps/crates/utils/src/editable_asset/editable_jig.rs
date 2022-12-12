use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::jig::{
    AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive, JigPlayerSettings,
    TextDirection,
};
use shared::domain::meta::AffiliationId;
use shared::domain::module::body::ThemeId;
use shared::domain::{
    category::CategoryId,
    jig::{JigId, JigResponse, JigUpdateDraftDataRequest},
    meta::AgeRangeId,
    module::LiteModule,
};

#[derive(Clone)]
pub struct EditableJig {
    pub id: JigId,
    // cover and modules only for read
    pub cover: Mutable<Option<LiteModule>>,
    pub modules: MutableVec<LiteModule>,
    pub published_at: Mutable<Option<DateTime<Utc>>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
    pub theme: Mutable<ThemeId>,
    pub audio_background: Mutable<Option<AudioBackground>>,
    pub feedback_positive: Mutable<HashSet<AudioFeedbackPositive>>,
    pub feedback_negative: Mutable<HashSet<AudioFeedbackNegative>>,
    pub direction: Mutable<TextDirection>,
    pub display_score: Mutable<bool>,
    pub track_assessments: Mutable<bool>,
    pub drag_assist: Mutable<bool>,
}

impl From<JigResponse> for EditableJig {
    fn from(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            cover: Mutable::new(jig.jig_data.modules.first().cloned()),
            modules: MutableVec::new_with_values(jig.jig_data.modules),
            display_name: Mutable::new(jig.jig_data.display_name),
            description: Mutable::new(jig.jig_data.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                jig.jig_data.additional_resources,
            )),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
            published_at: Mutable::new(jig.published_at),
            theme: Mutable::new(jig.jig_data.theme),
            audio_background: Mutable::new(jig.jig_data.audio_background),
            feedback_positive: Mutable::new(jig.jig_data.audio_effects.feedback_positive),
            feedback_negative: Mutable::new(jig.jig_data.audio_effects.feedback_negative),
            direction: Mutable::new(jig.jig_data.default_player_settings.direction),
            display_score: Mutable::new(jig.jig_data.default_player_settings.display_score),
            track_assessments: Mutable::new(jig.jig_data.default_player_settings.track_assessments),
            drag_assist: Mutable::new(jig.jig_data.default_player_settings.drag_assist),
        }
    }
}

impl From<JigId> for EditableJig {
    fn from(jig_id: JigId) -> Self {
        Self {
            id: jig_id,
            cover: Default::default(),
            modules: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            age_ranges: Default::default(),
            language: Default::default(),
            categories: Default::default(),
            affiliations: Default::default(),
            additional_resources: Default::default(),
            privacy_level: Default::default(),
            published_at: Default::default(),
            theme: Default::default(),
            audio_background: Default::default(),
            feedback_positive: Default::default(),
            feedback_negative: Default::default(),
            direction: Default::default(),
            display_score: Default::default(),
            track_assessments: Default::default(),
            drag_assist: Default::default(),
        }
    }
}

impl EditableJig {
    pub fn fill_from_jig(&self, jig: JigResponse) {
        self.cover.set(jig.jig_data.modules.first().cloned());
        self.modules.lock_mut().replace_cloned(jig.jig_data.modules);
        self.display_name.set(jig.jig_data.display_name);
        self.description.set(jig.jig_data.description.clone());
        self.age_ranges
            .set(HashSet::from_iter(jig.jig_data.age_ranges));
        self.language.set(jig.jig_data.language);
        self.categories
            .set(HashSet::from_iter(jig.jig_data.categories));
        self.affiliations
            .set(HashSet::from_iter(jig.jig_data.affiliations));
        self.additional_resources
            .lock_mut()
            .replace_cloned(jig.jig_data.additional_resources);
        self.privacy_level.set(jig.jig_data.privacy_level);
        self.published_at.set(jig.published_at);
        self.theme.set(jig.jig_data.theme);
        self.audio_background.set(jig.jig_data.audio_background);
        self.feedback_positive
            .set(jig.jig_data.audio_effects.feedback_positive);
        self.feedback_negative
            .set(jig.jig_data.audio_effects.feedback_negative);
        self.direction
            .set(jig.jig_data.default_player_settings.direction);
        self.display_score
            .set(jig.jig_data.default_player_settings.display_score);
        self.track_assessments
            .set(jig.jig_data.default_player_settings.track_assessments);
        self.drag_assist
            .set(jig.jig_data.default_player_settings.drag_assist);
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            id: self.id,
            cover: Mutable::new(self.cover.get_cloned()),
            modules: MutableVec::new_with_values(self.modules.lock_ref().to_vec()),
            published_at: Mutable::new(self.published_at.get()),
            display_name: Mutable::new(self.display_name.get_cloned()),
            description: Mutable::new(self.description.get_cloned()),
            age_ranges: Mutable::new(self.age_ranges.get_cloned()),
            language: Mutable::new(self.language.get_cloned()),
            categories: Mutable::new(self.categories.get_cloned()),
            affiliations: Mutable::new(self.affiliations.get_cloned()),
            additional_resources: Rc::new(MutableVec::new_with_values(
                self.additional_resources.lock_ref().to_vec(),
            )),
            privacy_level: Mutable::new(self.privacy_level.get()),
            theme: Mutable::new(self.theme.get()),
            audio_background: Mutable::new(self.audio_background.get()),
            feedback_positive: Mutable::new(self.feedback_positive.get_cloned()),
            feedback_negative: Mutable::new(self.feedback_negative.get_cloned()),
            direction: Mutable::new(self.direction.get()),
            display_score: Mutable::new(self.display_score.get()),
            track_assessments: Mutable::new(self.track_assessments.get()),
            drag_assist: Mutable::new(self.drag_assist.get()),
        }
    }

    pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        JigUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            theme: Some(self.theme.get()),
            audio_background: Some(self.audio_background.get()),
            audio_effects: Some(AudioEffects {
                feedback_positive: self.feedback_positive.get_cloned(),
                feedback_negative: self.feedback_negative.get_cloned(),
            }),
            default_player_settings: Some(JigPlayerSettings {
                direction: self.direction.get(),
                display_score: self.display_score.get(),
                track_assessments: self.track_assessments.get(),
                drag_assist: self.drag_assist.get(),
            }),
            ..Default::default()
        }
    }
}
