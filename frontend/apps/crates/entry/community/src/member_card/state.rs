use dominator::Dom;

use shared::domain::user::{public_user::PublicUser, UserId};

// Keeping simple. No new method, Config, Rc. Might not be able to keep it this way
pub struct MemberCard<'a> {
    pub member: &'a PublicUser,
    // the id of the user that's viewing, not the member be
    pub current_user_id: Option<UserId>,
    pub menu: Option<Dom>,
    pub admin_tag: bool,
}
