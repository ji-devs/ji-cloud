use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures::join;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        category::CategoryId,
        jig::{
            additional_resource::AdditionalResource, JigFocus, JigId, JigRating, JigResponse,
            JigUpdateAdminDataRequest, JigUpdateDraftDataRequest, LiteModule, PrivacyLevel,
        },
        meta::AffiliationId,
        meta::AgeRangeId,
    },
    error::EmptyError,
};
use utils::prelude::api_with_auth_empty;

#[derive(Clone)]
pub struct EditableJig {
    pub id: JigId,
    // modules only for read
    pub modules: Vec<LiteModule>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub other_keywords: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
    pub rating: Mutable<Option<JigRating>>,
    pub blocked: Mutable<bool>,
    pub jig_focus: JigFocus,
    pub author_name: String,
    pub published_at: Option<DateTime<Utc>>,
    pub loader: AsyncLoader,
}

impl From<JigResponse> for EditableJig {
    fn from(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            modules: jig.jig_data.modules,
            display_name: Mutable::new(jig.jig_data.display_name),
            description: Mutable::new(jig.jig_data.description.clone()),
            other_keywords: Mutable::new(jig.jig_data.other_keywords.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                jig.jig_data.additional_resources,
            )),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
            rating: Mutable::new(jig.admin_data.rating),
            blocked: Mutable::new(jig.admin_data.blocked),
            jig_focus: jig.jig_focus,
            author_name: jig.author_name.unwrap_or_default(),
            published_at: jig.published_at,
            loader: AsyncLoader::new(),
        }
    }
}

impl EditableJig {
    pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        JigUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            other_keywords: Some(self.other_keywords.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            ..Default::default()
        }
    }

    pub fn to_update_admin_data_request(&self) -> JigUpdateAdminDataRequest {
        JigUpdateAdminDataRequest {
            rating: self.rating.get_cloned(),
            blocked: Some(self.blocked.get()),
            ..Default::default()
        }
    }

    pub async fn save_draft(self: &Rc<Self>) {
        let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &self.id.0.to_string());
        let req = self.to_jig_update_request();
        let res = api_with_auth_empty::<EmptyError, JigUpdateDraftDataRequest>(
            &path,
            endpoints::jig::UpdateDraftData::METHOD,
            Some(req),
        )
        .await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub async fn save_admin_data(self: &Rc<Self>) {
        let path = endpoints::jig::JigAdminDataUpdate::PATH.replace("{id}", &self.id.0.to_string());
        let req = self.to_update_admin_data_request();
        let res = api_with_auth_empty::<EmptyError, JigUpdateAdminDataRequest>(
            &path,
            endpoints::jig::JigAdminDataUpdate::METHOD,
            Some(req),
        )
        .await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub async fn publish(self: &Rc<Self>) {
        let path = endpoints::jig::Publish::PATH.replace("{id}", &self.id.0.to_string());
        let res =
            api_with_auth_empty::<EmptyError, ()>(&path, endpoints::jig::Publish::METHOD, None)
                .await;
        match res {
            Ok(res) => res,
            Err(_) => todo!(),
        }
    }

    pub fn save_and_publish(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.save_draft(),
                state.save_admin_data(),
            );
            state.publish().await;
        }))
    }
}
