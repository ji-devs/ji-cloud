use std::rc::Rc;

use components::asset_search_bar::AssetSearchBar;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{asset::Asset, course::CourseId, jig::JigResponse};
use utils::drag::Drag;

use crate::edit::AssetEditState;

pub struct CourseSelection {
    pub course_id: CourseId,
    pub search_bar: Rc<AssetSearchBar>,
    pub asset_edit_state: Rc<AssetEditState>,
    pub loader: AsyncLoader,
    pub search_results: MutableVec<Rc<JigResponse>>,
    pub drag: Mutable<Option<Rc<Drag<Asset>>>>,
}

impl CourseSelection {
    pub fn new(course_id: CourseId, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            course_id,
            search_bar: AssetSearchBar::new(),
            asset_edit_state: Rc::clone(asset_edit_state),
            loader: AsyncLoader::new(),
            search_results: Default::default(),
            drag: Default::default(),
        })
    }
}
