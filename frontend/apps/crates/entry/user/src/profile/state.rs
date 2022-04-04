use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    image::ImageId,
    meta::{AffiliationId, AgeRangeId, MetadataResponse, SubjectId},
    user::{PatchProfileRequest, UserProfile},
};
use uuid::Uuid;

pub struct ProfilePage {
    pub user: ProfilePageUser,
    pub active_popup: Mutable<ActivePopup>,
    pub reset_password_status: Mutable<ResetPasswordStatus>,
    pub loader: AsyncLoader,
    pub metadata: Mutable<Option<MetadataResponse>>,
}

impl ProfilePage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            user: ProfilePageUser::empty(),
            active_popup: Mutable::new(ActivePopup::None),
            reset_password_status: Mutable::new(ResetPasswordStatus::default()),
            loader: AsyncLoader::new(),
            metadata: Mutable::new(None),
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ResetPasswordStatus {
    Ready,
    Loading,
    Sent,
}

impl Default for ResetPasswordStatus {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    None,
    Affiliation,
    Subjects,
    Age,
}

#[derive(Debug)]
pub struct ProfilePageUser {
    pub id: Mutable<Uuid>,
    pub username: Mutable<String>,
    pub email: Mutable<String>,
    pub given_name: Mutable<String>,
    pub family_name: Mutable<String>,
    pub profile_image: Mutable<Option<ImageId>>,
    pub language: Mutable<String>,
    // pub locale: Mutable<String>,
    pub location: Mutable<Option<serde_json::Value>>,
    pub organization: Mutable<Option<String>>,
    pub subjects: MutableVec<SubjectId>,
    pub age_ranges: MutableVec<AgeRangeId>,
    pub affiliations: MutableVec<AffiliationId>,
    pub persona: MutableVec<String>,
}

impl ProfilePageUser {
    pub fn empty() -> Self {
        Self {
            id: Mutable::new(Uuid::from_u128(0)),
            username: Mutable::new(String::new()),
            email: Mutable::new(String::new()),
            given_name: Mutable::new(String::new()),
            family_name: Mutable::new(String::new()),
            profile_image: Mutable::new(None),
            language: Mutable::new(String::new()),
            // locale: Mutable::new(String::new()),
            location: Mutable::new(None),
            organization: Mutable::new(None),
            subjects: MutableVec::new(),
            age_ranges: MutableVec::new(),
            affiliations: MutableVec::new(),
            persona: MutableVec::new(),
        }
    }

    pub fn fill_from_user(&self, user: UserProfile) {
        self.id.set(user.id);
        self.username.set(user.username);
        self.email.set(user.email);
        self.given_name.set(user.given_name);
        self.family_name.set(user.family_name);
        self.profile_image.set(user.profile_image);
        self.language.set(user.language);
        //self.locale.set(user.locale);
        self.location.set(user.location);
        self.organization.set(user.organization);
        self.subjects.lock_mut().replace(user.subjects);
        self.age_ranges.lock_mut().replace(user.age_ranges);
        self.affiliations.lock_mut().replace(user.affiliations);
        self.persona.lock_mut().replace_cloned(user.persona);
    }

    pub fn to_update(&self) -> PatchProfileRequest {
        PatchProfileRequest {
            given_name: Some(self.given_name.get_cloned()),
            family_name: Some(self.family_name.get_cloned()),
            profile_image: Some(self.profile_image.get_cloned()),
            language: Some(self.language.get_cloned()),
            // locale: Some(self.locale.get_cloned()),
            organization: Some(self.organization.get_cloned()),
            subjects: Some(self.subjects.lock_ref().to_vec()),
            age_ranges: Some(self.age_ranges.lock_ref().to_vec()),
            affiliations: Some(self.affiliations.lock_ref().to_vec()),
            location: Some(self.location.get_cloned()),
            persona: Some(self.persona.lock_ref().to_vec()),
            ..Default::default()
        }
    }
}
