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
    course::{CourseId, CourseResponse, CourseUpdateDraftDataRequest},
    meta::AgeRangeId,
    module::LiteModule,
};

#[derive(Clone)]
pub struct EditableCourse {
    pub id: CourseId,
    // cover and modules only for read
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

impl From<CourseResponse> for EditableCourse {
    fn from(course: CourseResponse) -> Self {
        Self {
            id: course.id,
            cover: Mutable::new(course.course_data.cover),
            items: MutableVec::new_with_values(course.course_data.items),
            display_name: Mutable::new(course.course_data.display_name),
            description: Mutable::new(course.course_data.description),
            age_ranges: Mutable::new(HashSet::from_iter(course.course_data.age_ranges)),
            language: Mutable::new(course.course_data.language),
            categories: Mutable::new(HashSet::from_iter(course.course_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(course.course_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                course.course_data.additional_resources,
            )),
            privacy_level: Mutable::new(course.course_data.privacy_level),
            published_at: Mutable::new(course.published_at),
        }
    }
}

impl From<CourseId> for EditableCourse {
    fn from(course_id: CourseId) -> Self {
        Self {
            id: course_id,
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

impl EditableCourse {
    pub fn fill_from_course(&self, course: CourseResponse) {
        self.cover.set(course.course_data.cover);
        self.items.lock_mut().replace(course.course_data.items);
        self.display_name.set(course.course_data.display_name);
        self.description.set(course.course_data.description.clone());
        self.age_ranges
            .set(HashSet::from_iter(course.course_data.age_ranges));
        self.language.set(course.course_data.language);
        self.categories
            .set(HashSet::from_iter(course.course_data.categories));
        self.affiliations
            .set(HashSet::from_iter(course.course_data.affiliations));
        self.additional_resources
            .lock_mut()
            .replace_cloned(course.course_data.additional_resources);
        self.privacy_level.set(course.course_data.privacy_level);
        self.published_at.set(course.published_at);
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

    pub fn to_course_update_request(&self) -> CourseUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        CourseUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            ..Default::default()
        }
    }
}
