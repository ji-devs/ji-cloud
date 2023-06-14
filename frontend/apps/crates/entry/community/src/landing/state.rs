use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{circle::Circle, course::CourseResponse, user::public_user::PublicUser};

pub struct CommunityLanding {
    pub top_members: Mutable<Option<Vec<PublicUser>>>,
    pub top_circles: Mutable<Option<Vec<Circle>>>,
    pub top_courses: Mutable<Option<Vec<CourseResponse>>>,
    pub loader: AsyncLoader,
}

impl CommunityLanding {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            top_members: Mutable::new(None),
            top_circles: Mutable::new(None),
            top_courses: Mutable::new(None),
        })
    }
}
