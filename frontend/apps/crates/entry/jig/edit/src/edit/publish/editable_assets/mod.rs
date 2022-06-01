mod editable_course;
mod editable_jig;

use std::{collections::HashSet, rc::Rc};

use chrono::{DateTime, Utc};
pub use editable_course::EditableCourse;
pub use editable_jig::EditableJig;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    additional_resource::AdditionalResource,
    asset::{AssetId, PrivacyLevel},
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId},
    module::LiteModule,
};

pub enum EditableAsset {
    Jig(EditableJig),
    Course(EditableCourse),
}

impl EditableAsset {
    pub fn id(&self) -> AssetId {
        match self {
            EditableAsset::Jig(jig) => jig.id.into(),
            EditableAsset::Course(course) => course.id.into(),
        }
    }

    pub fn cover(&self) -> &Option<LiteModule> {
        match self {
            EditableAsset::Jig(jig) => &jig.cover,
            EditableAsset::Course(course) => &course.cover,
        }
    }

    pub fn display_name(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.display_name,
            EditableAsset::Course(course) => &course.display_name,
        }
    }

    pub fn description(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.description,
            EditableAsset::Course(course) => &course.description,
        }
    }

    pub fn age_ranges(&self) -> &Mutable<HashSet<AgeRangeId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.age_ranges,
            EditableAsset::Course(course) => &course.age_ranges,
        }
    }

    pub fn language(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.language,
            EditableAsset::Course(course) => &course.language,
        }
    }

    pub fn categories(&self) -> &Mutable<HashSet<CategoryId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.categories,
            EditableAsset::Course(course) => &course.categories,
        }
    }

    pub fn affiliations(&self) -> &Mutable<HashSet<AffiliationId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.affiliations,
            EditableAsset::Course(course) => &course.affiliations,
        }
    }

    pub fn additional_resources(&self) -> &Rc<MutableVec<AdditionalResource>> {
        match self {
            EditableAsset::Jig(jig) => &jig.additional_resources,
            EditableAsset::Course(course) => &course.additional_resources,
        }
    }

    pub fn privacy_level(&self) -> &Mutable<PrivacyLevel> {
        match self {
            EditableAsset::Jig(jig) => &jig.privacy_level,
            EditableAsset::Course(course) => &course.privacy_level,
        }
    }

    pub fn published_at(&self) -> &Option<DateTime<Utc>> {
        match self {
            EditableAsset::Jig(jig) => &jig.published_at,
            EditableAsset::Course(course) => &course.published_at,
        }
    }

    pub fn _is_jig(&self) -> bool {
        match self {
            EditableAsset::Jig(jig) => jig.jig_focus.is_modules(),
            _ => false,
        }
    }

    pub fn is_resource(&self) -> bool {
        match self {
            EditableAsset::Jig(jig) => jig.jig_focus.is_resources(),
            _ => false,
        }
    }

    pub fn _is_course(&self) -> bool {
        matches!(self, Self::Course(_))
    }
}
