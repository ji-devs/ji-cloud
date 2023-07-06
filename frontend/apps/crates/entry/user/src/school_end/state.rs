use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::billing::SchoolId;
use std::rc::Rc;
use utils::{prelude::get_school_id, unwrap::UnwrapJiExt};
use web_sys::File;

pub struct SchoolEnd {
    pub school_id: SchoolId,
    pub loader: AsyncLoader,
    pub description: Mutable<Option<String>>,
    pub profile_image: Mutable<Option<File>>,
    pub website: Mutable<Option<String>>,
    pub organization_type: Mutable<Option<String>>,
}
impl SchoolEnd {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            school_id: get_school_id().unwrap_ji(),
            loader: AsyncLoader::new(),
            description: Mutable::new(None),
            profile_image: Mutable::new(None),
            website: Mutable::new(None),
            organization_type: Mutable::new(None),
        })
    }
}
