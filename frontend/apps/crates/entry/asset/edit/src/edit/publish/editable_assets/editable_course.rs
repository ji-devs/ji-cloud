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
    pub cover: Option<LiteModule>,
    pub items: Vec<JigId>,
    pub published_at: Option<DateTime<Utc>>,
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
            cover: course.course_data.cover,
            items: course.course_data.items,
            display_name: Mutable::new(course.course_data.display_name),
            description: Mutable::new(course.course_data.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(course.course_data.age_ranges)),
            language: Mutable::new(course.course_data.language),
            categories: Mutable::new(HashSet::from_iter(course.course_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(course.course_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                course.course_data.additional_resources,
            )),
            privacy_level: Mutable::new(course.course_data.privacy_level),
            published_at: course.published_at,
        }
    }
}

impl EditableCourse {
    pub fn new(course: CourseResponse) -> Self {
        Self {
            id: course.id,
            display_name: Mutable::new(course.course_data.display_name),
            cover: course.course_data.cover,
            items: course.course_data.items,
            description: Mutable::new(course.course_data.description),
            age_ranges: Mutable::new(HashSet::from_iter(course.course_data.age_ranges)),
            language: Mutable::new(course.course_data.language),
            categories: Mutable::new(HashSet::from_iter(course.course_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(course.course_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                course.course_data.additional_resources,
            )),
            privacy_level: Mutable::new(course.course_data.privacy_level),
            published_at: course.published_at,
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
