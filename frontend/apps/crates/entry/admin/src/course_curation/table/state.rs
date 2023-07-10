use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::course_curation::CourseCuration;

pub struct CourseTable {
    pub loader: AsyncLoader,
    pub curation_state: Rc<CourseCuration>,
}

impl CourseTable {
    pub fn new(curation_state: Rc<CourseCuration>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
