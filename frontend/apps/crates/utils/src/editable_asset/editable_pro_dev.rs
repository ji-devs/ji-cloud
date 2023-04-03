use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::pro_dev::unit::ProDevUnit;
use shared::domain::{
    category::CategoryId,
    module::LiteModule,
    pro_dev::{ProDevId, ProDevResponse, ProDevUpdateDraftDataRequest},
};

#[derive(Clone)]
pub struct EditableProDev {
    pub id: ProDevId,
    // cover and modules only for read
    pub cover: Mutable<Option<LiteModule>>,
    pub units: MutableVec<ProDevUnit>,
    pub published_at: Mutable<Option<DateTime<Utc>>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub language: Mutable<String>,
    pub duration: Mutable<Option<u32>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
}

impl From<ProDevResponse> for EditableProDev {
    fn from(pro_dev: ProDevResponse) -> Self {
        Self {
            id: pro_dev.id,
            cover: Mutable::new(pro_dev.pro_dev_data.cover),
            units: MutableVec::new_with_values(pro_dev.pro_dev_data.units),
            display_name: Mutable::new(pro_dev.pro_dev_data.display_name),
            description: Mutable::new(pro_dev.pro_dev_data.description),
            language: Mutable::new(pro_dev.pro_dev_data.language),
            categories: Mutable::new(HashSet::from_iter(pro_dev.pro_dev_data.categories)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                pro_dev.pro_dev_data.additional_resources,
            )),
            privacy_level: Mutable::new(pro_dev.pro_dev_data.privacy_level),
            published_at: Mutable::new(pro_dev.published_at),
            duration: Mutable::new(pro_dev.pro_dev_data.duration),
        }
    }
}

impl From<ProDevId> for EditableProDev {
    fn from(pro_dev_id: ProDevId) -> Self {
        Self {
            id: pro_dev_id,
            cover: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            language: Default::default(),
            categories: Default::default(),
            additional_resources: Default::default(),
            privacy_level: Default::default(),
            published_at: Default::default(),
            units: Default::default(),
            duration: Default::default(),
        }
    }
}

impl EditableProDev {
    pub fn fill_from_pro_dev(&self, pro_dev: ProDevResponse) {
        self.cover.set(pro_dev.pro_dev_data.cover);
        self.units
            .lock_mut()
            .replace_cloned(pro_dev.pro_dev_data.units);
        self.display_name.set(pro_dev.pro_dev_data.display_name);
        self.description
            .set(pro_dev.pro_dev_data.description.clone());
        self.language.set(pro_dev.pro_dev_data.language);
        self.categories
            .set(HashSet::from_iter(pro_dev.pro_dev_data.categories));
        self.additional_resources
            .lock_mut()
            .replace_cloned(pro_dev.pro_dev_data.additional_resources);
        self.privacy_level.set(pro_dev.pro_dev_data.privacy_level);
        self.published_at.set(pro_dev.published_at);
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            id: self.id,
            cover: Mutable::new(self.cover.get_cloned()),
            units: MutableVec::new_with_values(self.units.lock_ref().to_vec()),
            published_at: Mutable::new(self.published_at.get()),
            display_name: Mutable::new(self.display_name.get_cloned()),
            description: Mutable::new(self.description.get_cloned()),
            language: Mutable::new(self.language.get_cloned()),
            categories: Mutable::new(self.categories.get_cloned()),
            additional_resources: Rc::new(MutableVec::new_with_values(
                self.additional_resources.lock_ref().to_vec(),
            )),
            privacy_level: Mutable::new(self.privacy_level.get()),
            duration: Mutable::new(self.duration.get()),
        }
    }

    pub fn to_pro_dev_update_request(&self) -> ProDevUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        ProDevUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            // not updating because it'll override the existing units, need a better solution
            // units: Some(self.units.lock_ref().to_vec()),
            ..Default::default()
        }
    }
}
