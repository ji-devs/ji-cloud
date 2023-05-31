use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::jig::JigId;
use shared::domain::meta::AffiliationId;
use shared::domain::{
    category::CategoryId,
    meta::AgeRangeId,
    module::LiteModule,
    playlist::{PlaylistId, PlaylistResponse, PlaylistUpdateDraftDataRequest},
};

#[derive(Clone)]
pub struct EditablePlaylist {
    pub id: PlaylistId,
    pub cover: Mutable<Option<LiteModule>>,
    pub items: MutableVec<JigId>,
    pub published_at: Mutable<Option<DateTime<Utc>>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
}

impl From<PlaylistResponse> for EditablePlaylist {
    fn from(playlist: PlaylistResponse) -> Self {
        Self {
            id: playlist.id,
            cover: Mutable::new(playlist.playlist_data.cover),
            items: MutableVec::new_with_values(playlist.playlist_data.items),
            display_name: Mutable::new(playlist.playlist_data.display_name),
            description: Mutable::new(playlist.playlist_data.description),
            age_ranges: Mutable::new(HashSet::from_iter(playlist.playlist_data.age_ranges)),
            language: Mutable::new(playlist.playlist_data.language),
            categories: Mutable::new(HashSet::from_iter(playlist.playlist_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(playlist.playlist_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                playlist.playlist_data.additional_resources,
            )),
            privacy_level: Mutable::new(playlist.playlist_data.privacy_level),
            published_at: Mutable::new(playlist.published_at),
        }
    }
}

impl From<PlaylistId> for EditablePlaylist {
    fn from(playlist_id: PlaylistId) -> Self {
        Self {
            id: playlist_id,
            cover: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            age_ranges: Default::default(),
            language: Default::default(),
            categories: Default::default(),
            affiliations: Default::default(),
            additional_resources: Default::default(),
            privacy_level: Default::default(),
            published_at: Default::default(),
            items: Default::default(),
        }
    }
}

impl EditablePlaylist {
    pub fn fill_from_playlist(&self, playlist: PlaylistResponse) {
        self.cover.set(playlist.playlist_data.cover);
        self.items.lock_mut().replace(playlist.playlist_data.items);
        self.display_name.set(playlist.playlist_data.display_name);
        self.description
            .set(playlist.playlist_data.description.clone());
        self.age_ranges
            .set(HashSet::from_iter(playlist.playlist_data.age_ranges));
        self.language.set(playlist.playlist_data.language);
        self.categories
            .set(HashSet::from_iter(playlist.playlist_data.categories));
        self.affiliations
            .set(HashSet::from_iter(playlist.playlist_data.affiliations));
        self.additional_resources
            .lock_mut()
            .replace_cloned(playlist.playlist_data.additional_resources);
        self.privacy_level.set(playlist.playlist_data.privacy_level);
        self.published_at.set(playlist.published_at);
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            id: self.id,
            cover: Mutable::new(self.cover.get_cloned()),
            items: MutableVec::new_with_values(self.items.lock_ref().to_vec()),
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
        }
    }

    pub fn to_playlist_update_request(&self) -> PlaylistUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        PlaylistUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            // not updating because it'll override the existing items, need a better solution
            // items: Some(self.items.lock_ref().to_vec()),
            ..Default::default()
        }
    }
}
