use dominator::Dom;
use futures_signals::signal::ReadOnlyMutable;
use shared::domain::user::public_user::PublicUser;

// Keeping simple. No new method, Config, Rc. Might not be able to keep it this way
pub struct MemberCard<'a> {
    pub member: &'a PublicUser,
    pub slot: &'a str,
    pub menu: Option<Dom>,
    pub following: ReadOnlyMutable<bool>,
    pub on_follow: Box<dyn Fn(bool)>,
}
