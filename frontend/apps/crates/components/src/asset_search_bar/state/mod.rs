use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use shared::domain::{
    course::CourseSearchQuery, jig::JigSearchQuery, resource::ResourceSearchQuery,
};

mod search_state;

pub use search_state::SearchSelected;

pub struct AssetSearchBar {
    pub search_selected: Rc<SearchSelected>,
    pub loader: AsyncLoader,
}

impl AssetSearchBar {
    pub fn new() -> Rc<Self> {
        Self::new_with_search_selected(Default::default())
    }

    pub fn new_with_search_selected(search_selected: SearchSelected) -> Rc<Self> {
        Rc::new(Self {
            search_selected: Rc::new(search_selected),
            loader: AsyncLoader::new(),
        })
    }

    pub fn get_search_request_jig(&self) -> JigSearchQuery {
        self.search_selected.to_jig_search_request()
    }

    pub fn get_search_request_course(&self) -> CourseSearchQuery {
        self.search_selected.to_course_search_request()
    }

    pub fn get_search_request_resource(&self) -> ResourceSearchQuery {
        self.search_selected.to_resource_search_request()
    }
}

#[derive(Clone, Debug)]
pub struct AssetSearchQuery {
    pub jig: JigSearchQuery,
    pub course: CourseSearchQuery,
    pub resource: ResourceSearchQuery,
}
