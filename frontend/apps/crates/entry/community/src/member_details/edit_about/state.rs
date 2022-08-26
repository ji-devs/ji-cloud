use std::{collections::HashSet, rc::Rc};

use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use itertools::Itertools;
use shared::domain::user::UserProfile;

use crate::member_details::callbacks::EditProfileCallbacks;

pub struct EditAbout {
    user: UserProfile,
    pub callbacks: EditProfileCallbacks,
    pub location: Mutable<Option<serde_json::Value>>,
    pub location_public: Mutable<bool>,
    pub organization: Mutable<Option<String>>,
    pub organization_public: Mutable<bool>,
    pub persona: MutableVec<String>,
    pub persona_public: Mutable<bool>,
    pub languages_spoken: Mutable<HashSet<String>>,
    pub languages_spoken_public: Mutable<bool>,
}

impl EditAbout {
    pub fn new(user: UserProfile, callbacks: EditProfileCallbacks) -> Rc<Self> {
        Rc::new(Self {
            callbacks,
            location: Mutable::new(user.location.clone()),
            location_public: Mutable::new(user.location_public),
            organization: Mutable::new(user.organization.clone()),
            organization_public: Mutable::new(user.organization_public),
            persona: MutableVec::new_with_values(user.persona.clone()),
            persona_public: Mutable::new(user.persona_public),
            languages_spoken: Mutable::new(HashSet::from_iter(user.languages_spoken.clone())),
            languages_spoken_public: Mutable::new(user.languages_spoken_public),
            user,
        })
    }

    pub fn get_user_profile_from_fields(&self) -> UserProfile {
        let mut user = self.user.clone();

        user.location = self.location.get_cloned();
        user.location_public = self.location_public.get();
        user.organization = self.organization.get_cloned();
        user.organization_public = self.organization_public.get();
        user.persona = self.persona.lock_ref().to_vec();
        user.persona_public = self.persona_public.get();
        user.languages_spoken = self.languages_spoken.get_cloned().into_iter().collect_vec();
        user.languages_spoken_public = self.languages_spoken_public.get();

        user
    }
}
