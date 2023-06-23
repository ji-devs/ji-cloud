use dominator::Dom;

use shared::domain::user::public_user::PublicUser;

// Keeping simple. No new method, Config, Rc. Might not be able to keep it this way
pub struct MemberCard<'a> {
    pub member: &'a PublicUser,
    pub menu: Option<Dom>,
    pub admin_tag: bool,
}
