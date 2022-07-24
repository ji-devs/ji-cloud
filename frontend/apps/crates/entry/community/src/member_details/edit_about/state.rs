use std::rc::Rc;

use futures_signals::{signal::Mutable, signal_vec::MutableVec};
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
    pub language: Mutable<String>,
    pub language_public: Mutable<bool>,
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
            language: Mutable::new(user.language.clone()),
            language_public: Mutable::new(user.language_public),
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
        user.language = self.language.get_cloned();
        user.language_public = self.language_public.get();

        user
    }
}
