use std::rc::Rc;

use chrono::{DateTime, Utc};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::billing::{AmountInCents, AppliedCoupon, SubscriptionStatus};
use shared::domain::{
    billing::{PaymentMethodType, SchoolId},
    image::ImageId,
    meta::{AffiliationId, AgeRangeId, MetadataResponse, SubjectId},
    user::{PatchProfileRequest, UserId, UserProfile},
};

pub struct SettingsPage {
    pub user: SettingsPageUser,
    pub active_popup: Mutable<ActivePopup>,
    pub reset_password_status: Mutable<ResetPasswordStatus>,
    pub loader: AsyncLoader,
    pub metadata: Mutable<Option<MetadataResponse>>,
    pub(super) plan_info: Mutable<Option<Rc<PlanSectionInfo>>>,
    pub portal_link: Mutable<Option<String>>,
}

impl SettingsPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            user: SettingsPageUser::empty(),
            active_popup: Mutable::new(ActivePopup::None),
            reset_password_status: Mutable::new(ResetPasswordStatus::default()),
            loader: AsyncLoader::new(),
            metadata: Mutable::new(None),
            plan_info: Default::default(),
            portal_link: Default::default(),
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
pub struct SettingsPageUser {
    pub id: Mutable<UserId>,
    pub username: Mutable<String>,
    pub email: Mutable<String>,
    pub given_name: Mutable<String>,
    pub family_name: Mutable<String>,
    pub profile_image: Mutable<Option<ImageId>>,
    pub language_app: Mutable<String>,
    pub language_emails: Mutable<String>,
    pub location: Mutable<Option<serde_json::Value>>,
    pub organization: Mutable<Option<String>>,
    pub subjects: MutableVec<SubjectId>,
    pub age_ranges: MutableVec<AgeRangeId>,
    pub affiliations: MutableVec<AffiliationId>,
    pub persona: MutableVec<String>,
}

impl SettingsPageUser {
    pub fn empty() -> Self {
        Self {
            id: Mutable::new(UserId::from_u128(0)),
            username: Mutable::new(String::new()),
            email: Mutable::new(String::new()),
            given_name: Mutable::new(String::new()),
            family_name: Mutable::new(String::new()),
            profile_image: Mutable::new(None),
            language_app: Mutable::new(String::new()),
            language_emails: Mutable::new(String::new()),
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
        self.language_app.set(user.language_app);
        self.language_emails.set(user.language_emails);
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
            language_app: Some(self.language_app.get_cloned()),
            language_emails: Some(self.language_emails.get_cloned()),
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

#[derive(Clone)]
pub(super) struct PlanSectionInfo {
    pub auto_renew: Mutable<bool>,
    pub status: SubscriptionStatus,
    pub is_trial: bool,
    pub payment_method_type: Option<PaymentMethodType>,
    pub price: AmountInCents,
    pub coupon: Option<AppliedCoupon>,
    pub current_period_end: DateTime<Utc>,
    pub individual_or_school: IndividualOrSchool,
}

#[derive(strum_macros::EnumIs, Clone)]
pub(super) enum IndividualOrSchool {
    Individual,
    School(SchoolId),
}
