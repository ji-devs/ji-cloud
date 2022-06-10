use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{jig::JigResponse, user::public_user::PublicUser};
use uuid::Uuid;

pub struct MemberDetails {
    pub member_id: Uuid,
    pub member: Mutable<Option<PublicUser>>,
    pub loader: AsyncLoader,
    pub creations: Mutable<Creations>,
}

impl MemberDetails {
    pub fn new(member_id: Uuid) -> Rc<Self> {
        Rc::new(Self {
            member_id,
            member: Mutable::new(None),
            loader: AsyncLoader::new(),
            creations: Mutable::new(Creations::Jigs(None)),
        })
    }
}

pub enum Creations {
    Jigs(Option<Vec<JigResponse>>),
    Resources(Option<Vec<JigResponse>>),
    // Courses(Option<Vec<CourseResponse>>),
}
