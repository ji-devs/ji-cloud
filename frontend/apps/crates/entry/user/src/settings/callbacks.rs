use shared::domain::user::UserProfile;

pub struct EditProfileCallbacks {
    pub save_changes: Box<dyn Fn(UserProfile)>,
    pub close: Box<dyn Fn()>,
}
