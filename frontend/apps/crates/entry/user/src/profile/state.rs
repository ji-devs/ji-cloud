use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{meta::{AffiliationId, AgeRangeId, MetadataResponse, SubjectId}, user::UserProfile};
use dominator_helpers::futures::AsyncLoader;
use uuid::Uuid;

pub struct State {
    pub user: ProfilePageUser,
    pub active_popup: Mutable<ActivePopup>,
    pub loader: AsyncLoader,
    pub metadata: Mutable<Option<MetadataResponse>>,
}

impl State {
    pub fn new() -> Self {
        Self { 
            user: ProfilePageUser::empty(),
            active_popup: Mutable::new(ActivePopup::None),
            loader: AsyncLoader::new(),
            metadata: Mutable::new(None),
        }
    }
}


#[derive(Clone)]
pub enum ActivePopup {
    None,
    ResetPassword,
    Affiliation,
    Subjects,
    Age
}

#[derive(Debug)]
pub struct ProfilePageUser {
    pub id: Mutable<Uuid>,
    pub username: Mutable<String>,
    pub email: Mutable<String>,
    pub given_name: Mutable<String>,
    pub family_name: Mutable<String>,
    pub language: Mutable<String>,
    // pub locale: Mutable<String>,
    pub location: Mutable<Option<serde_json::Value>>,
    pub organization: Mutable<Option<String>>,
    pub subjects: MutableVec<SubjectId>,
    // pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub age_ranges: MutableVec<AgeRangeId>,
    pub affiliations: MutableVec<AffiliationId>,
}

impl ProfilePageUser {
    pub fn empty() -> Self {
        Self {
            id: Mutable::new(Uuid::from_u128(0)),
            username: Mutable::new(String::new()),
            email: Mutable::new(String::new()),
            given_name: Mutable::new(String::new()),
            family_name: Mutable::new(String::new()),
            language: Mutable::new(String::new()),
            // locale: Mutable::new(String::new()),
            location: Mutable::new(None),
            organization: Mutable::new(None),
            subjects: MutableVec::new(),
            // age_ranges: Mutable::new(HashSet::new()),
            age_ranges: MutableVec::new(),
            affiliations: MutableVec::new(),
        }
    }

    pub fn fill_from_user(&self, user: UserProfile) {
        self.id.set(user.id);
        self.username.set(user.username);
        self.email.set(user.email);
        self.given_name.set(user.given_name);
        self.family_name.set(user.family_name);
        self.language.set(user.language);
        //self.locale.set(user.locale);
        self.location.set(user.location);
        self.organization.set(user.organization);
        self.subjects.lock_mut().replace(user.subjects);
        self.age_ranges.lock_mut().replace(user.age_ranges);
        // self.age_ranges.set(HashSet::from_iter(user.age_ranges));
        self.affiliations.lock_mut().replace(user.affiliations);
    }

    pub fn to_update(&self) -> String {
        // TODO: Make this right
        format!("{:?}", self)
    }

}
