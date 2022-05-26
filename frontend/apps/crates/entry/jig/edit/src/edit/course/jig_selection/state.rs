use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::{course::CourseId, jig::JigId};

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
