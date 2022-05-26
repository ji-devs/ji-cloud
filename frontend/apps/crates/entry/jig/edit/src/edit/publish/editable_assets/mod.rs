mod editable_jig;

use std::{collections::HashSet, rc::Rc};

use chrono::{DateTime, Utc};
pub use editable_jig::*;
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
}

impl EditableAsset {
    pub fn id(&self) -> AssetId {
        match self {
            EditableAsset::Jig(jig) => jig.id.into(),
        }
    }

    pub fn cover(&self) -> &Option<LiteModule> {
        match self {
            EditableAsset::Jig(jig) => &jig.cover,
        }
    }

    pub fn display_name(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.display_name,
        }
    }

    pub fn description(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.description,
        }
    }

    pub fn age_ranges(&self) -> &Mutable<HashSet<AgeRangeId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.age_ranges,
        }
    }

    pub fn language(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.language,
        }
    }

    pub fn categories(&self) -> &Mutable<HashSet<CategoryId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.categories,
        }
    }

    pub fn affiliations(&self) -> &Mutable<HashSet<AffiliationId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.affiliations,
        }
    }

    pub fn additional_resources(&self) -> &Rc<MutableVec<AdditionalResource>> {
        match self {
            EditableAsset::Jig(jig) => &jig.additional_resources,
        }
    }

    pub fn privacy_level(&self) -> &Mutable<PrivacyLevel> {
        match self {
            EditableAsset::Jig(jig) => &jig.privacy_level,
        }
    }

    pub fn published_at(&self) -> &Option<DateTime<Utc>> {
        match self {
            EditableAsset::Jig(jig) => &jig.published_at,
        }
    }

    pub fn _is_jig(&self) -> bool {
        match self {
            EditableAsset::Jig(jig) => jig.jig_focus.is_modules(),
        }
    }

    pub fn is_resource(&self) -> bool {
        match self {
            EditableAsset::Jig(jig) => jig.jig_focus.is_resources(),
        }
    }
}
