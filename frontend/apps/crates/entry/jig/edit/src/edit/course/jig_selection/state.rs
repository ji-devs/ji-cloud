use std::{rc::Rc, cell::RefCell};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::{jig::JigId, course::CourseId};

pub struct JigSelection {
    pub course_id: CourseId,
    pub input: RefCell<String>,
    pub jigs: MutableVec<JigId>,
    pub loader: AsyncLoader,
}

impl JigSelection {
    pub fn new(course_id: CourseId) -> Rc<Self> {
        Rc::new(Self {
            course_id,
            input: RefCell::new(String::new()),
            jigs: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
