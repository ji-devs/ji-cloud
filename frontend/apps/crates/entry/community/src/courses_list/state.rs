use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::course::{CourseId, CourseResponse};

pub struct CoursesList {
    pub items_per_page: u32,
    pub courses: Mutable<Option<Vec<CourseResponse>>>,
    pub loader: AsyncLoader,
    pub total_pages: Mutable<u32>,
    pub active_page: Mutable<u32>,
    pub total_course_count: Mutable<Option<u32>>,
    pub play_course: Mutable<Option<CourseId>>,
}

impl CoursesList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            items_per_page: 20,
            courses: Mutable::new(None),
            loader: AsyncLoader::new(),
            total_pages: Mutable::new(0),
            active_page: Mutable::new(1),
            total_course_count: Mutable::new(None),
            play_course: Mutable::new(None),
        })
    }
}
