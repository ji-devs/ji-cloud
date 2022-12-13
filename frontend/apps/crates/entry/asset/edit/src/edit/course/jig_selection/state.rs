use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{course::CourseId, jig::JigResponse};
use utils::drag::Drag;

use crate::edit::AssetEditState;

pub struct JigSelection {
    pub course_id: CourseId,
    pub input: RefCell<String>,
    pub asset_edit_state: Rc<AssetEditState>,
    pub loader: AsyncLoader,
    pub search_results: MutableVec<Rc<JigResponse>>,
    pub drag: Mutable<Option<Rc<Drag>>>,
}

impl JigSelection {
    pub fn new(course_id: CourseId, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            course_id,
            input: RefCell::new(String::new()),
            asset_edit_state: Rc::clone(asset_edit_state),
            loader: AsyncLoader::new(),
            search_results: Default::default(),
            drag: Default::default(),
        })
    }
}
