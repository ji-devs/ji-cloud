use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::user::UserProfile;

use crate::member_details::callbacks::EditProfileCallbacks;

pub struct EditBio {
    user: UserProfile,
    pub callbacks: EditProfileCallbacks,
    pub bio: Mutable<String>,
    pub bio_public: Mutable<bool>,
}

impl EditBio {
    pub fn new(user: UserProfile, callbacks: EditProfileCallbacks) -> Rc<Self> {
        Rc::new(Self {
            callbacks,
            bio: Mutable::new(user.bio.clone()),
            bio_public: Mutable::new(user.bio_public),
            user,
        })
    }

    pub fn get_user_profile_from_fields(&self) -> UserProfile {
        let mut user = self.user.clone();

        user.bio = self.bio.get_cloned();
        user.bio_public = self.bio_public.get();

        user
    }
}
